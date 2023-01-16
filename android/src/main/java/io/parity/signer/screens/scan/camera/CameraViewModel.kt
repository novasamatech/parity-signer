package io.parity.signer.screens.scan.camera

import android.annotation.SuppressLint
import android.util.Log
import androidx.camera.core.ImageProxy
import androidx.lifecycle.ViewModel
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.models.encodeHex
import io.parity.signer.models.submitErrorState
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow


class CameraViewModel() : ViewModel() {

	val isTorchEnabled = MutableStateFlow(false)

	private val _isBananaPassword = MutableStateFlow(false)
	val isBananaPassword: StateFlow<Boolean> = _isBananaPassword.asStateFlow()

	private val _pendingPayloads = MutableStateFlow<Set<String>>(emptySet())
	val pendingTransactionPayloads: StateFlow<Set<String>> =
		_pendingPayloads.asStateFlow()

	private val _total = MutableStateFlow<Int?>(null)
	private val _captured = MutableStateFlow<Int?>(null)

	// Observables for model data
	internal val total: StateFlow<Int?> = _total.asStateFlow()
	internal val captured: StateFlow<Int?> = _captured.asStateFlow()

	// payload of currently scanned qr codes for multiqr transaction like metadata update.
	private var currentMultiQrTransaction = arrayOf<String>()

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
			imageProxy.imageInfo.rotationDegrees,
		)

		barcodeScanner.process(inputImage)
			.addOnSuccessListener { barcodes ->
				barcodes.forEach {
					val payloadString = it?.rawBytes?.encodeHex()
					if (!(currentMultiQrTransaction.contains(payloadString) || payloadString.isNullOrEmpty())) {
						if (total.value == null) {
							try {
								val proposeTotal =
									qrparserGetPacketsTotal(payloadString, true).toInt()
								if (proposeTotal == 1) {
									decode(listOf(payloadString))
								} else {
									currentMultiQrTransaction += payloadString
									_total.value = proposeTotal
								}
							} catch (e: java.lang.Exception) {
								Log.e("scanVM", "QR sequence length estimation $e")
							}
						} else {
							currentMultiQrTransaction += payloadString
							if ((currentMultiQrTransaction.size + 1) >= (total.value ?: 0)) {
								decode(currentMultiQrTransaction.toList())
							} else {
								_captured.value = currentMultiQrTransaction.size
							}
							Log.d("scanVM", "captured " + captured.value.toString())
						}
					}
				}
			}
			.addOnFailureListener {
				Log.e("scanVM", "Scan failed " + it.message.toString())
			}
			.addOnCompleteListener {
				imageProxy.close()
			}
	}

	private fun decode(completePayload: List<String>) {
		try {
			val payload = qrparserTryDecodeQrSequence(
				data = completePayload,
				password = null,
				cleaned = true,
			)
			when (payload) {
				is DecodeSequenceResult.BBananaSplitRecoveryResult -> {
					when (val bananaResult = payload.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							//we passed a null in qrparserTryDecodeQrSequence so we can't get there
							submitErrorState("cannot happen here that for scanning we don't have password request")
						}
						BananaSplitRecoveryResult.RequestPassword -> {
							_isBananaPassword.value = true
							resetPendingTransactions() 				//todo banana
						}
					}
				}
				is DecodeSequenceResult.Other -> {
					val actualPayload = payload.s
					resetScanValues()
					addPendingTransaction(actualPayload)
				}
			}

		} catch (e: Exception) {
			Log.e("scanVM", "Single frame decode failed $e")
		}
	}

	private fun addPendingTransaction(payload: String) {
		_pendingPayloads.value += payload
	}

	/**
	 * Clears camera progress
	 */
	fun resetScanValues() {
		currentMultiQrTransaction = arrayOf()
		_captured.value = null
		_total.value = null
	}

	fun resetPendingTransactions() {
		_pendingPayloads.value = emptySet()
		resetScanValues()
	}

}
