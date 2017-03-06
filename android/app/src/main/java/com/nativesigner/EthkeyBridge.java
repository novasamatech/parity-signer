package com.nativesigner;

import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Promise;

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
    public void brainWalletAddress(String seed, Promise promise) {
        int a = hello();
        promise.resolve("hello from java " + a + " sa");
    }

    @ReactMethod
    public void brainWalletSecret(String seed, Promise promise) {
        promise.resolve("hello from java");
    }

    private static native int hello();
}
