package io.parity.signer.screens.logs.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.red400
import io.parity.signer.ui.theme.textTertiary

@Composable
fun LogItem(
	model: LogsListEntryModel.LogEntryModel,
	onClick: Callback,
) {
	Column(
		Modifier
			.fillMaxWidth(1f)
			.clickable(onClick = onClick)
			.padding(vertical = 8.dp, horizontal = 24.dp)
	) {
		Text(
			text = model.title,
			color = if (model.isDanger) {
				MaterialTheme.colors.red400
			} else {
				MaterialTheme.colors.primary
			},
			style = TypefaceNew.TitleS,
		)
		Spacer(modifier = Modifier.padding(top = 8.dp))
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			Text(
				text = model.message,
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
				modifier = Modifier.weight(1f)
			)
			Spacer(modifier = Modifier.padding(start = 8.dp))
			Text(
				text = model.timeStr,
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
			)
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
			)
		}
	}
}

@Composable
fun LogItemDate(model: LogsListEntryModel.TimeSeparatorModel) {
	Text(
		text = model.dateStr,
		color = MaterialTheme.colors.textTertiary,
		style = TypefaceNew.BodyM,
		modifier = Modifier.padding(vertical = 8.dp, horizontal = 24.dp)
	)
}

sealed class LogsListEntryModel {
	data class LogEntryModel(
		//id of this group of events, not unique per event
		val logGroupId: UInt,
		val title: String,
		val message: String,
		val timeStr: String,
		val isDanger: Boolean,
	) : LogsListEntryModel()

	data class TimeSeparatorModel(
		val dateStr: String,
	) : LogsListEntryModel()
}

@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewLogItem() {
	SignerNewTheme {
		Column() {
			LogItemDate(LogsListEntryModel.TimeSeparatorModel("Jun 20"))
			LogItem(
				LogsListEntryModel.LogEntryModel(
					title = "Database initiated",
					message = "Message of database init happened very long 2 lines",
					timeStr = "10:42",
					isDanger = false,
					logGroupId = 23.toUInt(),
				)
			) {}
		}
	}
}
