package io.parity.signer.modals

import android.widget.ImageView
import android.widget.Toast
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import org.json.JSONObject
import kotlin.math.roundToInt
import kotlin.math.sign

@Composable
fun ExportPublicKey(signerDataModel: SignerDataModel) {
	val address = signerDataModel.screenData.value ?: JSONObject()

	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
	) {
		KeyCard(identity = address)
		NetworkCard(address)
		Image(
			address.optString("qr").intoImageBitmap(),
			contentDescription = "QR with address to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.fillMaxWidth(1f)
		)
		HeaderBar(line1 = "KEY DETAILS", line2 = "")
		Row {
			Text("Base58 key:")
			Text(address.optString("base58"))
		}
		Row {
			Text("Hex key:")
			Text(address.optString("pubkey"))
		}
		Row {
			Text("Seed name:")
			Text(address.optString("seed_name"))
		}
	}
}
