require 'json'
package_json = JSON.parse(File.read('../../node_modules/react-native-secure-storage/package.json'))

Pod::Spec.new do |s|

  s.name           = "RNSecureStorage"
  s.version        = package_json["version"]
  s.summary        = "use secure storage in react native"
  s.homepage       = "https://github.com/paritytech/react-native-secure-storage"
  s.license        = package_json["license"]
  s.author         = { package_json["author"] => package_json["author"] }
  s.platform       = :ios, "9.0"
  s.source         = { :git => "git@github.com:paritytech/react-native-secure-storage.git"}
  s.source_files   = '../../node_modules/react-native-secure-storage/ios/*.{h,m}'
  s.dependency 'React'

end
