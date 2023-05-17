require 'xcodeproj'

project_path = File.join(ENV['PROJECT_DIR'], 'PolkadotVault.xcodeproj')
framework_path = File.join(ENV['PROJECT_DIR'], 'Frameworks', 'signer.xcframework')

project = Xcodeproj::Project.open(project_path)
target = project.targets.find { |t| t.name == 'PolkadotVault' }

existing_framework_file = target.frameworks_build_phase.files_references.find do |f|
  unless f.nil?
    File.basename(f.real_path.to_s) == 'Frameworks/' + File.basename(framework_path)
  end
end

if existing_framework_file.nil?
  main_group = project.main_group
  framework_group = main_group.find_subpath('Frameworks', true)
  if framework_group.nil?
    framework_group = main_group.new_group('Frameworks', 'Frameworks')
  end

  framework_file = framework_group.new_file(framework_path)
  framework_file.name = File.basename(framework_path)
  framework_file.source_tree = '<group>'
  framework_file.path = 'Frameworks/' + File.basename(framework_path)
  target.frameworks_build_phase.add_file_reference(framework_file)
  project.save
end
