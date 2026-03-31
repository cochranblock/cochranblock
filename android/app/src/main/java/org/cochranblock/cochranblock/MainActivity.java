package org.cochranblock.cochranblock;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.webkit.WebSettings;
import java.io.*;

public class MainActivity extends Activity {
    private WebView webView;
    private Process serverProcess;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Extract and start the Rust server binary
        startServer();

        // Create WebView pointing to localhost
        webView = new WebView(this);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(false); // zero JS site
        settings.setDomStorageEnabled(true);
        webView.setWebViewClient(new WebViewClient());
        setContentView(webView);

        // Wait for server to bind, then load
        new Thread(() -> {
            try { Thread.sleep(2000); } catch (Exception e) {}
            runOnUiThread(() -> webView.loadUrl("http://127.0.0.1:8081"));
        }).start();
    }

    private void startServer() {
        try {
            // Copy native lib to executable location
            String nativeDir = getApplicationInfo().nativeLibraryDir;
            String binaryPath = nativeDir + "/libcochranblock.so";

            File binary = new File(binaryPath);
            if (!binary.exists()) return;

            // Make executable
            binary.setExecutable(true);

            // Start server process
            ProcessBuilder pb = new ProcessBuilder(binaryPath);
            pb.environment().put("PORT", "8081");
            pb.environment().put("BIND", "127.0.0.1");
            pb.redirectErrorStream(true);
            serverProcess = pb.start();

        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Override
    public void onBackPressed() {
        if (webView != null && webView.canGoBack()) {
            webView.goBack();
        } else {
            super.onBackPressed();
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        if (serverProcess != null) {
            serverProcess.destroy();
        }
    }
}
