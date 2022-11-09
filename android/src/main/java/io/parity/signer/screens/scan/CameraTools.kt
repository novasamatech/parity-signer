package io.parity.signer.screens.scan

import androidx.compose.runtime.Composable

@Composable
private fun RequestCameraPermission() {
	//todo dmitry
//	when {
//		ContextCompat.checkSelfPermission(
//			LocalContext.current,
//			Manifest.permission.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION
//		) == PackageManager.PERMISSION_GRANTED -> {
//			Log.i("kilo", "Permission previously granted")
//		}
//
//		ActivityCompat.shouldShowRequestPermissionRationale(
//			LocalContext.current,
//			Manifest.permission.CAMERA
//		) -> Log.i("kilo", "Show camera permissions dialog")
//
//		else -> {
//			registerForActivityResult(
//				ActivityResultContracts.RequestPermission()
//			) { isGranted ->
//				if (isGranted) {
//					Log.i("kilo", "Permission granted")
//				} else {
//					Log.i("kilo", "Permission denied")
//				}
//			}
//		}
//	}
}
