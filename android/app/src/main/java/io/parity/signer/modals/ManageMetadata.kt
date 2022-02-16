package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.MetadataCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import org.json.JSONObject

@Composable
fun ManageMetadata(signerDataModel: SignerDataModel) {
	val content = signerDataModel.modalData.value ?: JSONObject()
	var confirm by remember { mutableStateOf(false) }

	Column(
		modifier = Modifier.padding(20.dp)
	) {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column {
				HeaderBar(line1 = "MANAGE METADATA", line2 = "Select action")
				MetadataCard(content)
				Row {
					Text("Used for:")
					LazyColumn {
						items(content.optJSONArray("networks")?.length() ?: 0) { index ->
							NetworkCard(
								network = content.getJSONArray("networks").getJSONObject(index)
							)
						}
					}
				}
				BigButton(
					text = "Sign this metadata",
					isShaded = true,
					isCrypto = true,
					action = { signerDataModel.pushButton(ButtonID.SignMetadata) })
				BigButton(
					text = "Delete this metadata",
					isShaded = true,
					isDangerous = true,
					action = {
						confirm = true
					}
				)
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Remove metadata?",
		text = "This metadata will be removed for all networks",
		back = { confirm = false },
		forward = { signerDataModel.pushButton(ButtonID.RemoveMetadata) },
		backText = "Cancel",
		forwardText = "Remove metadata"
	)
}
