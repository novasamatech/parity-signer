package io.parity.signer;

import org.junit.Test;

import static org.junit.Assert.*;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.WritableMap;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.test.core.app.ApplicationProvider;

class TestPromise implements Promise {
    private Object expectedValue;
    private boolean shouldBeRejected;

    public TestPromise(@Nullable Object expectedValue, boolean shouldBeRejected) {
        this.expectedValue = expectedValue;
        this.shouldBeRejected = shouldBeRejected;
    }

    public void resolve(@Nullable Object value) {
        assertEquals(value, this.expectedValue);
    }

    public void reject(String code, String message) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, Throwable throwable) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, String message, Throwable throwable) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(Throwable throwable) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(Throwable throwable, WritableMap userInfo) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, @NonNull WritableMap userInfo) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, Throwable throwable, WritableMap userInfo) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, String message, @NonNull WritableMap userInfo) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String code, String message, Throwable throwable, WritableMap userInfo) {
        assertTrue(this.shouldBeRejected);
    }

    public void reject(String message) {
        assertTrue(this.shouldBeRejected);
    }
}

public class EthkeyBridgeTest {
    private String TEST_APP = "TEST_APP";
    private String NOT_TEST_APP = "NOT_TEST_APP";
    private String TEST_KEY = "TEST_KEY";
    private String NOT_TEST_KEY = "NOT_TEST_KEY";
    private String TEST_PIN = "42";

    ReactApplicationContext reactApplicationContext = new ReactApplicationContext(ApplicationProvider.getApplicationContext());
    EthkeyBridge ethkey = new EthkeyBridge(reactApplicationContext);

//    @Test
//    public void testFailsWithoutBiometricAuth() {
//        this.ethkey.secureContains(TEST_APP, TEST_KEY, new TestPromise(false, false));
//        this.ethkey.securePut(TEST_APP, TEST_KEY, TEST_PIN, false, new TestPromise(null, false));
//        this.ethkey.secureContains(TEST_APP, TEST_KEY, new TestPromise(true, false));
//    }
}
