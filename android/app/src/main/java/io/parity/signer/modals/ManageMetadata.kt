package io.parity.signer.modals

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
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
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MNetworkDetails
import io.parity.signer.uniffi.MmManageNetworks
import io.parity.signer.uniffi.MscNetworkInfo

@Composable
fun ManageMetadata(
	networks: MmManageNetworks,
	signerDataModel: SignerDataModel
) {
	var confirm by remember { mutableStateOf(false) }

	Surface(
		color = Color.Transparent,
		modifier = Modifier.clickable { signerDataModel.pushButton(Action.GO_BACK) }
	) {
		Column {
			Spacer(Modifier.weight(1f))
			Surface(
				color = MaterialTheme.colors.Bg000,
				shape = MaterialTheme.shapes.modal
			) {
				Column(
					modifier = Modifier.padding(20.dp)
				) {
					HeaderBar(line1 = "MANAGE METADATA", line2 = "Select action")

					Row {
						Text("Used for:")
						LazyColumn {
							items(networks.networks.size) { index ->
								NetworkCard(
									network = MscNetworkInfo(
										networkTitle = networks.networks[index].title,
										networkLogo = networks.networks[index].logo
									)
								)
							}
						}
					}
					BigButton(
						text = "Sign this metadata",
						isShaded = true,
						isCrypto = true,
						action = { signerDataModel.pushButton(Action.SIGN_METADATA) })
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
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Remove metadata?",
		text = "This metadata will be removed for all networks",
		back = { confirm = false },
		forward = { signerDataModel.pushButton(Action.REMOVE_METADATA) },
		backText = "Cancel",
		forwardText = "Remove metadata"
	)
}
