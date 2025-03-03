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

  profile_name = "match AppStore io.parity.NativeSigner"
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
  app_identifier = CredentialsManager::AppfileConfig.try_fetch_value(:app_identifier)

  if is_ci?
    create_keychain(
      name: "github_actions_keychain",
      password: ENV["KEYCHAIN_PASSWORD"],
      default_keychain: true,
      unlock: true,
      timeout: 3600,
      add_to_search_list: true,
      lock_when_sleeps: false
    )
  end

  match(
    type: "appstore",
    app_identifier: app_identifier,
    readonly: false,
    keychain_name: "github_actions_keychain",
    keychain_password: ENV["KEYCHAIN_PASSWORD"]
  )

end
