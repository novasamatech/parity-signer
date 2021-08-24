package io.parity.signer.screens

import android.widget.Toast
import androidx.compose.animation.expandHorizontally
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import io.parity.signer.MainActivity
import io.parity.signer.models.SignerDataModel

@Composable
fun HomeScreen(signerDataModel: SignerDataModel, navToTransaction: () -> Unit) {
	Box(
		modifier = Modifier
			.clickable(onClick = navToTransaction)
			.fillMaxSize()
	) {
		Image(
			painter = painterResource(id = io.parity.signer.R.drawable.icon),
			contentDescription = "Icon"
		)
	}
}

