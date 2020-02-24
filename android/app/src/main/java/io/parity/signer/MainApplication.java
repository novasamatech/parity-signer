package io.parity.signer;

import android.app.Application;

import com.facebook.react.ReactApplication;
import com.th3rdwave.safeareacontext.SafeAreaContextPackage;
import com.reactlibrary.RNSecureStoragePackage;
import org.reactnative.camera.RNCameraPackage;
import com.swmansion.gesturehandler.react.RNGestureHandlerPackage;
import com.horcrux.svg.SvgPackage;
import com.bitgo.randombytes.RandomBytesPackage;
import com.reactnativecommunity.netinfo.NetInfoPackage;
import com.facebook.react.ReactNativeHost;
import com.facebook.react.ReactPackage;
import com.facebook.react.shell.MainReactPackage;
import com.facebook.soloader.SoLoader;


import java.util.Arrays;
import java.util.List;

public class MainApplication extends Application implements ReactApplication {

  private final ReactNativeHost mReactNativeHost = new ReactNativeHost(this) {
    @Override
    public boolean getUseDeveloperSupport() {
      return BuildConfig.DEBUG;
    }

    @Override
    protected List<ReactPackage> getPackages() {
      return Arrays.<ReactPackage>asList(
          new MainReactPackage(),
            new SafeAreaContextPackage(),
          new RNSecureStoragePackage(),
          new RNCameraPackage(),
          new SvgPackage(),
          new NetInfoPackage(),
          new RandomBytesPackage(),
          new EthkeyBridgePackage(),
          new RNGestureHandlerPackage()
      );
    }

    @Override
    protected String getJSMainModuleName() {
      return "index";
    }
  };

  @Override
  public ReactNativeHost getReactNativeHost() {
    return mReactNativeHost;
  }

  @Override
  public void onCreate() {
    super.onCreate();
    SoLoader.init(this, /* native exopackage */ false);
  }
}
