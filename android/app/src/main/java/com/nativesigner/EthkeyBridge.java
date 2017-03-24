package com.nativesigner;

import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Promise;

/**
 * Created by marek on 20/02/2017.
 */

public class EthkeyBridge extends ReactContextBaseJavaModule {
    static {
        System.loadLibrary("signer");
    }

    @Override
    public String getName() {
        return "EthkeyBridge";
    }

    public EthkeyBridge(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @ReactMethod
    public void brainWalletAddress(String seed, Promise promise) {
        promise.resolve(ethkeyBrainwalletAddress(seed));
    }

    @ReactMethod
    public void brainWalletSecret(String seed, Promise promise) {
        promise.resolve(ethkeyBrainwalletSecret(seed));
    }

    @ReactMethod
    public void brainWalletSign(String seed, String message, Promise promise) {
        promise.resolve(ethkeyBrainwalletSign(seed, message));
    }

    @ReactMethod
    public void rlpItem(String rlp, int position, Promise promise) {
        try {
            promise.resolve(ethkeyRlpItem(rlp, position));
        } catch (Exception e) {
            promise.reject("invalid rlp", null, null);

        }
    }

    @ReactMethod
    public void keccak(String data, Promise promise) {
        promise.resolve(ethkeyKeccak(data));
    }

    @ReactMethod
    public void blockiesIcon(String seed, Promise promise) {
        promise.resolve(ethkeyBlockiesIcon(seed));
    }

    private static native String ethkeyBrainwalletAddress(String seed);
    private static native String ethkeyBrainwalletSecret(String seed);
    private static native String ethkeyBrainwalletSign(String seed, String message);
    private static native String ethkeyRlpItem(String data, int position);
    private static native String ethkeyKeccak(String data);
    private static native String ethkeyBlockiesIcon(String seed);
}