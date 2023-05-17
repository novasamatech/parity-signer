require 'xcodeproj'

project_path = File.join(ENV['PROJECT_DIR'], 'PolkadotVault.xcodeproj')
framework_path = File.join(ENV['PROJECT_DIR'], 'Frameworks', 'signer.xcframework')

project = Xcodeproj::Project.open(project_path)
target = project.targets.find { |t| t.name == 'PolkadotVault' }

framework_build_phase = target.frameworks_build_phase

existing_framework_file = framework_build_phase.files_references.find do |f|
  f&.path.to_s == 'Frameworks/' + File.basename(framework_path)
end

if existing_framework_file.nil?
  framework_file = project.new(Xcodeproj::Project::Object::PBXFileReference)
  framework_file.name = File.basename(framework_path)
  framework_file.path = 'Frameworks/' + File.basename(framework_path)
  framework_file.source_tree = 'SOURCE_ROOT'
  framework_build_phase.add_file_reference(framework_file)
end

project.save
