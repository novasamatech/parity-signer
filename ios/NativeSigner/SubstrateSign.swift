import Foundation

func handle_error<T, U>(
  resolve: RCTPromiseResolveBlock,
  reject: RCTPromiseRejectBlock,
  get_result: (UnsafeMutablePointer<ExternError>) -> T,
  success: (T) -> U
) -> Void {
  var err = ExternError()
  let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
  let res = get_result(err_ptr)
  if err_ptr.pointee.code == 0 {
    resolve(success(res))
  } else {
    let val = String(cString: err_ptr.pointee.message)
    signer_destroy_string(err_ptr.pointee.message)
    reject(String(describing: err_ptr.pointee.code), val, nil)
  }
}

@objc(SubstrateSign)
class SubstrateSign: NSObject {

  public static func requiresMainQueueSetup() -> Bool {
    return true;
  }

  @objc func brainWalletAddress(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { ethkey_brainwallet_address($0, seed) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func brainWalletSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { ethkey_brainwallet_sign($0, seed, message) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func rlpItem(_ rlp: String, position: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { rlp_item($0, rlp, position) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func keccak(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { keccak256($0, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func blake2b(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { blake($0, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func ethSign(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { eth_sign($0, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func blockiesIcon(_ seed: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { blockies_icon($0, seed) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func randomPhrase(_ wordsNumber:NSInteger, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { random_phrase($0, Int32(wordsNumber)) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func encryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { encrypt_data($0, data, password) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func decryptData(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { decrypt_data($0, data, password) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func qrCode(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { qrcode($0, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func qrCodeHex(_ data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { qrcode_hex($0, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateAddress(_ seed: String, prefix: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_brainwallet_address($0, seed, prefix) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateSign(_ seed: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_brainwallet_sign($0, seed, message) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func schnorrkelVerify(_ seed: String, message: String, signature: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { schnorrkel_verify($0, seed, message, signature) },
      // return a bool. no cleanup
      success: { return $0 })
  }

  @objc func decryptDataRef(_ data: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { decrypt_data_ref($0, data, password) },
      // return a long. no cleanup
      success: { return $0 })
  }

  @objc func destroyDataRef(_ data_ref: Int64, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { destroy_data_ref($0, data_ref) },
      // return zero. no cleanup
      success: { return 0 })
  }

  @objc func brainWalletSignWithRef(_ seed_ref: Int64, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { ethkey_brainwallet_sign_with_ref($0, seed_ref, message) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateSignWithRef(_ seed_ref: Int64, suri_suffix: String, message: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_brainwallet_sign_with_ref($0, seed_ref, suri_suffix, message) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func brainWalletAddressWithRef(_ seed_ref: Int64, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { brain_wallet_address_with_ref($0, seed_ref) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateAddressWithRef(_ seed_ref: Int64, suri_suffix: String, prefix: UInt32, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_address_with_ref($0, seed_ref, suri_suffix, prefix) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateSecret(_ suri: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_mini_secret_key($0, suri) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func substrateSecretWithRef(_ seed_ref: Int64, suri_suffix: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { substrate_mini_secret_key_with_ref($0, seed_ref, suri_suffix) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func tryDecodeQrSequence(_ size: NSInteger, chunk_size: NSInteger, data: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { try_decode_qr_sequence($0, Int32(size), Int32(chunk_size), data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func generateMetadataHandle(_ metadata: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { generate_metadata_handle($0, metadata) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func parseTransaction(_ payload: String, gen_hash: String, metadata: String, type_descriptor: String, identities: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { parse_transaction($0, payload, metadata, type_descriptor, identities) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func signTransaction(_ action: String, pin: String, password: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { sign_transaction($0, action, pin, password) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }

  @objc func developmentTest(_ input: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> Void {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { development_test($0, input) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        signer_destroy_string(res!)
        return val
    })
  }



}


