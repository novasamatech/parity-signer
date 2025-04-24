desc "Submit a new build to Firebase"
lane :upload_firebase do
	firebase_app_distribution(
    groups: ENV["FIREBASE_GROUPS"],
    release_notes: ENV["FIREBASE_RELEASE_NOTES"]
  )
end