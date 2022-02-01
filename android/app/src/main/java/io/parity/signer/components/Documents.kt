package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Tab
import androidx.compose.material.TabRow
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier

//TODO: everything
@Composable
fun Documents() {
	var document by remember { mutableStateOf( 0 ) }
	Column {
		//Note to designer:
		//to make the selector pretty, implement
		//custom Tab:
		//https://developer.android.com/reference/kotlin/androidx/compose/material/package-summary#TabRow(kotlin.Int,androidx.compose.ui.Modifier,androidx.compose.ui.graphics.Color,androidx.compose.ui.graphics.Color,kotlin.Function1,kotlin.Function0,kotlin.Function0)
		TabRow(selectedTabIndex = document) {
			Tab(content = {Text("Terms of service")}, selected = document == 0, onClick = {document = 0})
			Tab(content = {Text("Privacy policy")}, selected = document == 1, onClick = {document = 1})
		}
		Column(Modifier.verticalScroll(rememberScrollState())) {
			when(document) {
				0 -> {
					Text("InstructionsSquare")
					Text("Terms and conditions here")
				}
				1 -> {
					Text("Privacy policy")
				}
				else -> {
					Text("document selection error")
				}
			}
		}
	}
}
