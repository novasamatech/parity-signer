package io.parity.signer.components

import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.Text
import androidx.compose.material.TopAppBar
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel

@Composable
fun TopBar(signerDataModel: SignerDataModel) {
	TopAppBar(
		title = { Text("Parity Signer") },
		navigationIcon = {
			IconButton(onClick = {
				signerDataModel.goBack()
			}) {
				Icon(Icons.Default.ArrowBack, contentDescription = "go back")
			}
		}
	)
}
