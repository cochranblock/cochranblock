package org.cochranblock.cochranblock;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.webkit.WebSettings;
import java.net.HttpURLConnection;
import java.net.URL;

public class MainActivity extends Activity {
    private WebView webView;

    private static native void startServer();
    private static boolean serverStarted = false;

    static {
        System.loadLibrary("cochranblock");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Start server only once (survives activity restart)
        if (!serverStarted) {
            startServer();
            serverStarted = true;
        }

        webView = new WebView(this);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(false); // zero JS — nav is CSS-only checkbox toggle
        settings.setDomStorageEnabled(true);
        webView.setWebViewClient(new WebViewClient());
        webView.setBackgroundColor(0xFF050508);
        setContentView(webView);

        // Poll until server responds, then load
        new Thread(() -> {
            for (int i = 0; i < 50; i++) {
                try {
                    HttpURLConnection c = (HttpURLConnection)
                        new URL("http://127.0.0.1:8081/health").openConnection();
                    c.setConnectTimeout(200);
                    c.setReadTimeout(200);
                    if (c.getResponseCode() == 200) {
                        runOnUiThread(() -> webView.loadUrl("http://127.0.0.1:8081"));
                        return;
                    }
                } catch (Exception e) { /* server not ready yet */ }
                try { Thread.sleep(100); } catch (Exception e) {}
            }
            // Fallback after 5 seconds
            runOnUiThread(() -> webView.loadUrl("http://127.0.0.1:8081"));
        }).start();
    }

    @Override
    public void onBackPressed() {
        if (webView != null && webView.canGoBack()) {
            webView.goBack();
        } else {
            super.onBackPressed();
        }
    }
}
