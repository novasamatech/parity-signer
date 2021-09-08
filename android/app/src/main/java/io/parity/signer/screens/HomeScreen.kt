package io.parity.signer.screens

import android.widget.Toast
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.animation.expandHorizontally
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.barcode.BarcodeScanning
import io.parity.signer.MainActivity
import io.parity.signer.TransactionState
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel

/**
 * This is a simple screen with a single button that
 * triggers transaction sequence starting with camera
 */
@Composable
fun HomeScreen(signerDataModel: SignerDataModel, navToTransaction: () -> Unit) {
	val transactionState = signerDataModel.transactionState.observeAsState()

	when(transactionState.value) {
		TransactionState.None -> {
			CameraModal(signerDataModel)
		}
		TransactionState.Parsing -> {
			WaitingScreen()
		}
		TransactionState.Preview -> {
			TransactionPreview(signerDataModel)
		}
		TransactionState.Password -> {
			TransactionPassword(signerDataModel)
		}
		TransactionState.Signed -> {
			TransactionSigned(signerDataModel)
		}
		null -> {
			WaitingScreen()
		}
	}

}

