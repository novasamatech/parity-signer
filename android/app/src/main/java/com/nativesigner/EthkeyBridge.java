package com.nativesigner;

import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Callback;

/**
 * Created by marek on 20/02/2017.
 */

public class EthkeyBridge extends ReactContextBaseJavaModule {
    @Override
    public String getName() {
        return "EthkeyBridge";
    }

    public EthkeyBridge(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @ReactMethod
    public void brainWalletAddress(String seed, Callback callback) {
        callback.invoke("hello from java");
    }

    @ReactMethod
    public void brainWalletSecret(String seed, Callback callback) {
        callback.invoke("hello from java");
    }
}
