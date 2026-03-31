package org.cochranblock.cochranblock;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.webkit.WebSettings;

public class MainActivity extends Activity {
    private WebView webView;

    // JNI — starts axum server on a background thread in Rust
    private static native void startServer();

    static {
        System.loadLibrary("cochranblock");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Start the Rust server via JNI (spawns background thread)
        startServer();

        // Create WebView pointing to localhost
        webView = new WebView(this);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(false);
        settings.setDomStorageEnabled(true);
        webView.setWebViewClient(new WebViewClient());
        setContentView(webView);

        // Wait for server to bind, then load
        new Thread(() -> {
            try { Thread.sleep(2000); } catch (Exception e) {}
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
