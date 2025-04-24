desc "Submit a new build to Firebase"
lane :upload_firebase do
	firebase_app_distribution(
		app: ENV["FIREBASE_APP_ID"],
    groups: ENV["FIREBASE_GROUPS"],
    release_notes: ENV["FIREBASE_RELEASE_NOTES"],
    service_credentials_file: ENV["FIREBASE_SERVICE_CREDENTIALS_FILE"]
  )
end