package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.Identicon
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MTypesInfo

@Composable
fun TypesInfo(
	typesInfo: MTypesInfo,
	button: (Action) -> Unit
) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "MANAGE TYPES", line2 = "Select action")
				if (typesInfo.typesOnFile) {
					Row {
						Identicon(identicon = typesInfo.typesIdPic?:listOf())
						Text(typesInfo.typesHash ?: "")
					}
				} else {
					Text("Pre-v14 types not installed")
				}
				BigButton(
					text = "Sign types",
					isShaded = true,
					isCrypto = true,
					action = { button(Action.SIGN_TYPES) })
				BigButton(
					text = "Delete types",
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
		header = "Remove types?",
		text = "Types information needed for support of pre-v14 metadata will be removed. Are you sure?",
		back = { confirm = false },
		forward = { button(Action.REMOVE_TYPES) },
		backText = "Cancel",
		forwardText = "Remove types"
	)
}
