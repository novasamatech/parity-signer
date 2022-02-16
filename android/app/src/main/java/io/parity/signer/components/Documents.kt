package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Tab
import androidx.compose.material.TabRow
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Typography

//TODO: everything
@Composable
fun Documents() {
	var document by remember { mutableStateOf(0) }
	Column {
		//Note to designer:
		//to make the selector pretty, implement
		//custom Tab:
		//https://developer.android.com/reference/kotlin/androidx/compose/material/package-summary#TabRow(kotlin.Int,androidx.compose.ui.Modifier,androidx.compose.ui.graphics.Color,androidx.compose.ui.graphics.Color,kotlin.Function1,kotlin.Function0,kotlin.Function0)
		TabRow(selectedTabIndex = document, Modifier.padding(horizontal = 20.dp)) {
			Tab(
				content = { Text("Terms of service", style = Typography.button) },
				selected = document == 0,
				onClick = { document = 0 })
			Tab(
				content = { Text("Privacy policy", style = Typography.button) },
				selected = document == 1,
				onClick = { document = 1 })
		}
		Column(
			Modifier
				.verticalScroll(rememberScrollState())
				.padding(20.dp)
		) {
			when (document) {
				0 -> {
					InstructionsSquare()
					TAC()
				}
				1 -> {
					PP()
				}
				else -> {
					Text("document selection error")
				}
			}
			Spacer(Modifier.height(150.dp))
		}
	}
}
