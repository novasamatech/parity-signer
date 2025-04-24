desc "Submit a new build to Apple TestFlight"
lane :upload_testflight do |options|
  upload_to_testflight(
    skip_waiting_for_build_processing: true,
    apple_id: "1218174838"
  )
end

