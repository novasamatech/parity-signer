package io.parity.signer.screens.scan.camera

import android.annotation.SuppressLint
import android.os.Trace
import androidx.camera.core.ImageProxy
import androidx.lifecycle.ViewModel
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.domain.encodeHex
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import timber.log.Timber


class CameraViewModel() : ViewModel() {

	val isTorchEnabled = MutableStateFlow(false)

	private val _bananaSplitPayload = MutableStateFlow<List<String>?>(null)
	val bananaSplitPayload: StateFlow<List<String>?> =
		_bananaSplitPayload.asStateFlow()

	private val _pendingTransactionPayloads =
		MutableStateFlow<Set<String>>(emptySet())
	val pendingTransactionPayloads: StateFlow<Set<String>> =
		_pendingTransactionPayloads.asStateFlow()

	private val _dynamicDerivationPayload =
		MutableStateFlow<String?>(null)
	val dynamicDerivationPayload: StateFlow<String?> =
		_dynamicDerivationPayload.asStateFlow()

	private val _dynamicDerivationTransactionPayload =
		MutableStateFlow<List<String>?>(null)
	val dynamicDerivationTransactionPayload: StateFlow<List<String>?> =
		_dynamicDerivationTransactionPayload.asStateFlow()

	private val _total = MutableStateFlow<Int?>(null)
	private val _captured = MutableStateFlow<Int?>(null)

	// Observables for model data
	internal val total: StateFlow<Int?> = _total.asStateFlow()
	internal val captured: StateFlow<Int?> = _captured.asStateFlow()

	// payload of currently scanned qr codes for multiqr transaction like metadata update.
	private var currentMultiQrTransaction = mutableSetOf<String>()

	/**
	 * Barcode detecting function.
	 * This uses experimental features
	 */
	@SuppressLint("UnsafeOptInUsageError")
	fun processFrame(
		barcodeScanner: BarcodeScanner,
		imageProxy: ImageProxy
	) {
		Trace.beginSection("process frame")
		if (imageProxy.image == null) return
		val inputImage = InputImage.fromMediaImage(
			imageProxy.image!!,
			imageProxy.imageInfo.rotationDegrees,
		)

		barcodeScanner.process(inputImage)
			.addOnSuccessListener { barcodes ->
				Trace.beginSection("process frame vault code")
				barcodes.forEach {
					val payloadString = it?.rawBytes?.encodeHex()
					if (!currentMultiQrTransaction.contains(payloadString) && !payloadString.isNullOrEmpty()) {
						if (total.value == null) {
							try {
								val proposeTotal =
									qrparserGetPacketsTotal(payloadString, true).toInt()
								if (proposeTotal == 1) {
									decode(listOf(payloadString))
								} else {
									currentMultiQrTransaction += payloadString
									_captured.value = currentMultiQrTransaction.size
									_total.value = proposeTotal
								}
							} catch (e: java.lang.Exception) {
								Timber.e("scanVM", "QR sequence length estimation $e")
							}
						} else {
							currentMultiQrTransaction += payloadString
							if ((currentMultiQrTransaction.size + 1) >= (total.value ?: 0)) {
								decode(currentMultiQrTransaction.toList())
							} else {
								_captured.value = currentMultiQrTransaction.size
							}
							Timber.d("scanVM", "captured " + captured.value.toString())
						}
					}
				}
				Trace.endSection()
			}
			.addOnFailureListener {
				Timber.e("scanVM", "Scan failed " + it.message.toString())
			}
			.addOnCompleteListener {
				Trace.endSection()
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
					when (payload.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							//we passed a null password in qrparserTryDecodeQrSequence so we can't get there
							submitErrorState("cannot happen here that for scanning we don't have password request")
						}

						BananaSplitRecoveryResult.RequestPassword -> {
							resetScanValues()
							_bananaSplitPayload.value = completePayload
						}
					}
				}

				is DecodeSequenceResult.Other -> {
					val actualPayload = payload.s
					resetScanValues()
					addPendingTransaction(actualPayload)
				}

				is DecodeSequenceResult.DynamicDerivations -> {
					resetScanValues()
					_dynamicDerivationPayload.value = payload.s
				}

				is DecodeSequenceResult.DynamicDerivationTransaction -> {
					resetScanValues()
					_dynamicDerivationTransactionPayload.value = payload.s
				}
			}

		} catch (e: Exception) {
			Timber.e("scanVM", "Single frame decode failed $e")
		}
	}

	private fun addPendingTransaction(payload: String) {
		_pendingTransactionPayloads.value += payload
	}

	/**
	 * Clears camera progress
	 */
	fun resetScanValues() {
		currentMultiQrTransaction = mutableSetOf()
		_captured.value = null
		_total.value = null
	}

	fun resetPendingTransactions() {
		_pendingTransactionPayloads.value = emptySet()
		_bananaSplitPayload.value = null
		_dynamicDerivationPayload.value = null
		_dynamicDerivationTransactionPayload.value = null
		resetScanValues()
	}
}
