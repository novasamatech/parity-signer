package io.parity.signer.screens

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.ArrowCircleDown
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.SignerModal
import io.parity.signer.components.KeySelector
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.SeedCard
import io.parity.signer.modals.*
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject

/**
 * Key manager screen; here all key/identity/seed creation and deletion
 * operations should happen. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun KeyManager(signerDataModel: SignerDataModel) {
	Column() {
		Row(
			Modifier
				.clickable {
					signerDataModel.pushButton(ButtonID.SelectKey, "")
				}
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.background(Bg200)
				.fillMaxWidth()
		) {
			SeedCard(
				seedName = "TODO",
				identicon = "",
				seedSelector = true,
				signerDataModel = signerDataModel
			)
		}
		IconButton(
			onClick = { signerDataModel.pushButton(ButtonID.NetworkSelector) },
			modifier = Modifier
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.background(Bg200)
				.fillMaxWidth()
		) {
			Row {
				//NetworkCard(signerDataModel.selectedNetwork.value?: JSONObject())
				Icon(Icons.Default.ArrowCircleDown, "More networks")
				Spacer(modifier = Modifier.weight(1f))
			}
		}
		Row(modifier = Modifier
			.fillMaxWidth(1f)
			.padding(horizontal = 8.dp)) {
			Text("DERIVED KEYS")
			Spacer(Modifier.weight(1f, true))
			IconButton(onClick = { /*TODO: export seed*/ }) {
				Icon(Icons.Default.AddCircle, contentDescription = "New derived key")
			}
		}
		KeySelector(signerDataModel)
	}
}

