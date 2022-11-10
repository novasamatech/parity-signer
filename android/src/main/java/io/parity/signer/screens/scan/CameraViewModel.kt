package io.parity.signer.screens.scan

import android.Manifest
import android.annotation.SuppressLint
import android.content.pm.PackageManager
import android.util.Log
import androidx.camera.core.ImageProxy
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.encodeHex
import io.parity.signer.models.navigate
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.qrparserGetPacketsTotal
import io.parity.signer.uniffi.qrparserTryDecodeQrSequence


class CameraViewModel(): ViewModel() {

	val navigator = EmptyNavigator() //todo dmitry do I need it?

	internal val _total = MutableLiveData<Int?>(null)
	internal val _captured = MutableLiveData<Int?>(null)
	internal val _progress = MutableLiveData(0.0f)
	// Observables for model data
	internal val total: LiveData<Int?> = _total
	internal val captured: LiveData<Int?> = _captured
	internal val progress: LiveData<Float> = _progress

	// Camera stuff
	internal var bucket = arrayOf<String>()
	internal var payload: String = ""


//	private val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
//	private val REQUEST_CODE_PERMISSIONS = 10
//	private fun allPermissionsGranted() = REQUIRED_PERMISSIONS.all {
//		ContextCompat.checkSelfPermission(
//			context, it
//		) == PackageManager.PERMISSION_GRANTED
//	}
//
//	internal fun handleCameraPermissions() {//todo dmitry remove
//		if (!allPermissionsGranted()) {
//			ActivityCompat.requestPermissions(
//				activity,
//				REQUIRED_PERMISSIONS,
//				REQUEST_CODE_PERMISSIONS
//			)
//		}
//	}

	/**
	 * Barcode detecting function.
	 * This uses experimental features
	 */
	@SuppressLint("UnsafeOptInUsageError")
	fun processFrame(
		barcodeScanner: BarcodeScanner,
		imageProxy: ImageProxy
	) {
		if (imageProxy.image == null) return
		val inputImage = InputImage.fromMediaImage(
			imageProxy.image!!,
			imageProxy.imageInfo.rotationDegrees
		)

		barcodeScanner.process(inputImage)
			.addOnSuccessListener { barcodes ->
				barcodes.forEach {
					val payloadString = it?.rawBytes?.encodeHex()
					if (!(bucket.contains(payloadString) || payloadString.isNullOrEmpty())) {
						if (total.value == null) {
							try {
								val proposeTotal =
									qrparserGetPacketsTotal(payloadString, true).toInt()
								if (proposeTotal == 1) {
									try {
										payload = qrparserTryDecodeQrSequence(
											listOf(payloadString),
											true
										)
										resetScanValues()
										navigator.navigate(Action.TRANSACTION_FETCHED, payload)
									} catch (e: java.lang.Exception) {
										Log.e("Single frame decode failed", e.toString())
									}
								} else {
									bucket += payloadString
									_total.value = proposeTotal
								}
							} catch (e: java.lang.Exception) {
								Log.e("QR sequence length estimation", e.toString())
							}
						} else {
							bucket += payloadString
							if ((bucket.size + 1) >= (total.value ?: 0)) {
								try {
									payload = qrparserTryDecodeQrSequence(
										bucket.toList(),
										true
									)
									if (payload.isNotEmpty()) {
										resetScanValues()
										navigator.navigate(Action.TRANSACTION_FETCHED, payload)
									}
								} catch (e: java.lang.Exception) {
									Log.e("failed to parse sequence", e.toString())
								}
							}
							_captured.value = bucket.size
							_progress.value = (
								(
									captured.value ?: 0
									).toFloat() / (
									(
										total.value
											?: 1
										).toFloat()
									)
								)
							Log.d("captured", captured.value.toString())
						}
					}
				}
			}
			.addOnFailureListener {
				Log.e("Scan failed", it.message.toString())
			}
			.addOnCompleteListener {
				imageProxy.close()
			}
	}

	/**
	 * Clears camera progress
	 */
	fun resetScanValues() {
		bucket = arrayOf()
		_captured.value = null
		_total.value = null
		_progress.value = 0.0f
	}

}
