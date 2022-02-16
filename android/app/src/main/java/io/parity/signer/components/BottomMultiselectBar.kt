package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.ui.theme.Text600

@Composable
fun BottomMultiselectBar(
	count: String,
	delete: () -> Unit,
	export: () -> Unit
) {
	var confirm by remember { mutableStateOf(false) }
	Row(
		horizontalArrangement = Arrangement.SpaceBetween,
		modifier = Modifier
			.height(40.dp)
			.fillMaxWidth()
			.padding(horizontal = 12.dp)
	) {
		SmallButton(text = "Delete") {
			confirm = true
		}
		Text(
			count + " items selected",
			style = MaterialTheme.typography.body2,
			color = MaterialTheme.colors.Text600
		)
		SmallButton(text = "Export", action = export)
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Delete keys?",
		text = "You are about to delete selected keys",
		back = { confirm = false },
		forward = delete,
		forwardText = "Delete"
	)
}
