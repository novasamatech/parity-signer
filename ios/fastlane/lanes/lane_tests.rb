desc "Tests given scheme"
desc "Parameters:"
desc "- 'scheme : <value>' to define scheme to test"
desc " "
desc "Example usage: fastlane test_build scheme:'NativeSigner'"
lane :test_build do |options|
  scheme = options[:scheme]
  clear_derived_data
  scan(
    clean: true,
    scheme: scheme,
    device: "iPhone 13",
    output_directory: "./fastlane/test_output/"
  )
end
