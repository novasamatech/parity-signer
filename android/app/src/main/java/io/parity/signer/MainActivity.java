package io.parity.signer;

import com.facebook.react.ReactActivity;
import com.facebook.react.ReactActivityDelegate;

import android.os.Bundle;
import android.view.WindowManager;

public class MainActivity extends ReactActivity {

    /**
     * Returns the name of the main component registered from JavaScript. This is used to schedule
     * rendering of the component.
     */
    @Override
    protected String getMainComponentName() {
        return "NativeSigner";
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (!BuildConfig.DEBUG) {
            getWindow().setFlags(WindowManager.LayoutParams.FLAG_SECURE,
                                       WindowManager.LayoutParams.FLAG_SECURE);
        }
    }

    @Override
    protected ReactActivityDelegate createReactActivityDelegate() {
        return new ReactActivityDelegate(this, getMainComponentName()) {
            @Override
            protected Bundle getLaunchOptions() {
                Bundle bundle = new Bundle();
                Bundle argumentsBundle = MainActivity.this.getIntent().getBundleExtra("launchArgs");
                bundle.putBundle("launchArgs", argumentsBundle);
                return bundle;
            }
        };
    }
}
