package io.parity.signer.screens.scan

import android.annotation.SuppressLint
import android.util.Log
import androidx.camera.core.ImageProxy
import androidx.lifecycle.ViewModel
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.encodeHex
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.qrparserGetPacketsTotal
import io.parity.signer.uniffi.qrparserTryDecodeQrSequence
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow


class CameraViewModel() : ViewModel() {

	val isMultiscanMode = MutableStateFlow(false)
	val isTourchEnabled = MutableStateFlow<Boolean>(false)

	private val _pendingPayloads = MutableStateFlow<List<String>>(emptyList())
	val pendingPayloads: StateFlow<List<String>> = _pendingPayloads.asStateFlow()

	private val _total = MutableStateFlow<Int?>(null)
	private val _captured = MutableStateFlow<Int?>(null)

	// Observables for model data
	internal val total: StateFlow<Int?> = _total.asStateFlow()
	internal val captured: StateFlow<Int?> = _captured.asStateFlow()


	// payload of currently scanned qr codes for multiqr transaction like metadata update.
	private var currentMultiQrTransaction = arrayOf<String>()

	private val uniffiInteractor = ServiceLocator.backendLocator.uniffiInteractor

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
									try {
										val payload = qrparserTryDecodeQrSequence(
											listOf(payloadString),
											true
										)
										resetScanValues()
										addPendingTransaction(payload)
									} catch (e: java.lang.Exception) {
										Log.e("Single frame decode failed", e.toString())
									}
								} else {
									currentMultiQrTransaction += payloadString
									_total.value = proposeTotal
								}
							} catch (e: java.lang.Exception) {
								Log.e("QR sequence length estimation", e.toString())
							}
						} else {
							currentMultiQrTransaction += payloadString
							if ((currentMultiQrTransaction.size + 1) >= (total.value ?: 0)) {
								try {
									val payload = qrparserTryDecodeQrSequence(
										currentMultiQrTransaction.toList(),
										true
									)
									if (payload.isNotEmpty()) {
										resetScanValues()
										addPendingTransaction(payload)
									}
								} catch (e: java.lang.Exception) {
									Log.e("failed to parse sequence", e.toString())
								}
							}
							_captured.value = currentMultiQrTransaction.size
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

	private fun addPendingTransaction(payload: String) {
		_pendingPayloads.value = _pendingPayloads.value + payload
	}

	suspend fun getTransactionsFromPendingPayload(): List<MTransaction> {
		val allResults = pendingPayloads.value.map { payload ->
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		}
		//todo handle error cases and show ot user?
		allResults.filterIsInstance<UniffiResult.Error<Any>>().forEach { error ->
			Log.e("Camera scan", "transaction parsing failed, ${error.error.message}")
		}
		return allResults.filterIsInstance<UniffiResult.Success<MTransaction>>()
			.map { it.result }
	}

	/**
	 * Clears camera progress
	 */
	fun resetScanValues() {
		currentMultiQrTransaction = arrayOf()
		_captured.value = null
		_total.value = null
	}

	sealed class CameraNavModel {
		object None : CameraNavModel()
		data class TransitionNavigation(val transactions: List<MTransaction>) :
			CameraNavModel()
	}
}
