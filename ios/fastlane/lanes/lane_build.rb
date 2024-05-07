desc "Build the iOS app for release"
desc "Parameters:"
desc "- 'scheme : <value>' defines scheme to use for build phase"
desc "- 'target : <value>' defines target to build"
desc "- 'configuration : <value>' defines configuration for build"
desc " "
desc "Example usage: fastlane build_release scheme:'NativeSigner' target: 'NativeSigner' configuration: 'Release' "
lane :build_release do |options|
  scheme = options[:scheme]
  target = options[:target]
  configuration = options[:configuration]
  app_identifier = CredentialsManager::AppfileConfig.try_fetch_value(:app_identifier)

  profile_name = "Polkadot Vault Distribution"
  output_name = scheme # just in case we need to customise it for other GAs
  export_method = "app-store"
  compile_bitcode = false
  xcodeproj_path = "./#{target}.xcodeproj"

  clean_build_artifacts

  increment_build_number(
    build_number: ENV["BUILD_NUMBER"], # based on commit, defined in GA
    xcodeproj: xcodeproj_path
  )
  update_code_signing_settings(
    use_automatic_signing: false,
    targets: [target],
    code_sign_identity: "Apple Distribution",
    bundle_identifier: app_identifier,
    profile_name: profile_name,
    build_configurations: [configuration]
  )
  gym(
    scheme: scheme,
    output_name: output_name,
    configuration: configuration,
    xcargs: "-skipPackagePluginValidation -skipMacroValidation",
    export_options: {
      method: export_method,
      provisioningProfiles: {
        app_identifier => profile_name
      },
      compileBitcode: compile_bitcode
    }
  )
end

desc "Prepares certificate and provisioning profile"
lane :prepare_code_signing do |options|
  api_key = lane_context[SharedValues::APP_STORE_CONNECT_API_KEY]
  app_identifier = CredentialsManager::AppfileConfig.try_fetch_value(:app_identifier)
  profile_name = "Polkadot Vault Distribution"

  cert(
    api_key: api_key,
    keychain_path: ENV["KEYCHAIN_PATH"],
    keychain_password: ENV["KEYCHAIN_PASSWORD"]
  )
  sigh(
    api_key: api_key,
    app_identifier: app_identifier,
    provisioning_name: profile_name,
    force: false
 )
end
