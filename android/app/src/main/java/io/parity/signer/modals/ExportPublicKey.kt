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
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.rememberSwipeableState
import androidx.compose.material.swipeable
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject
import kotlin.math.sign

@ExperimentalAnimationApi
@Composable
fun ExportPublicKey(signerDataModel: SignerDataModel) {
	val selectedIdentity = signerDataModel.selectedIdentity.observeAsState()
	val selectedNetwork = signerDataModel.selectedNetwork.observeAsState()

	Column(modifier = Modifier
		.fillMaxWidth()
		.draggable(
			orientation = Orientation.Horizontal,
			state = rememberDraggableState { delta ->
				if (delta > 10.0f) {
					Toast
						.makeText(signerDataModel.context, "swippeee", Toast.LENGTH_SHORT)
						.show()
				}
			}
		)
	) {
		Column(Modifier.padding(8.dp)) {
			AnimatedContent(
				targetState = selectedIdentity
			) {
				KeyCard(
					selectedIdentity.value ?: JSONObject(),
					signerDataModel = signerDataModel
				)
			}
			Spacer(Modifier.padding(8.dp))
			Text(selectedNetwork.value!!.get("title").toString())
		}
		AnimatedContent(
			targetState = selectedIdentity
		) {
			Image(
				bitmap = signerDataModel.exportPublicKey(),
				contentDescription = "QR with public key to scan",
				contentScale = ContentScale.FillWidth,
				modifier = Modifier.fillMaxWidth()
			)
		}
	}
}
