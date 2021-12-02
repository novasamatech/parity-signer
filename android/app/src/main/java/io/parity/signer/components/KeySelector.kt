package io.parity.signer.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getRootIdentity
import io.parity.signer.models.selectKey
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject

@Composable
fun KeySelector(signerDataModel: SignerDataModel) {
	val addresses = signerDataModel.screenInfo.getJSONArray("keys")
	LazyColumn {
		//keys should be defined already, can't panic
		items(addresses.length()) { item ->
			if (addresses.getJSONObject(item) != signerDataModel.getRootIdentity(
					signerDataModel.selectedSeed.value ?: ""
				)
			) {
				Row(
					verticalAlignment = Alignment.CenterVertically,
					modifier = Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.background(Bg200)
				) {
					Row(
						verticalAlignment = Alignment.CenterVertically,
						modifier = Modifier
							.pointerInput(Unit) {
								detectTapGestures(
									onTap = {
										signerDataModel.selectKey(
											addresses.getJSONObject(
												item
											) ?: JSONObject()
										)
									},
									onLongPress = {

									}
								)
							}
							.padding(horizontal = 8.dp)
					) {
						KeyCard(
							addresses.getJSONObject(item),
							signerDataModel = signerDataModel
						)
						Spacer(modifier = Modifier.weight(1f, true))
						Icon(Icons.Default.CheckCircle, "Address selected")
					}

				}
			}
		}
	}
}
