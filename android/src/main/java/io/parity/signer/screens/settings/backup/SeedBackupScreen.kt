package io.parity.signer.screens.settings.backup

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textSecondary


@Composable
fun SeedBackupScreen(
	seeds: List<String>,
	coreNavController: NavController,
	onBack: Callback,
	onSelected: (selectedSeed: String) -> Unit,
) {
	Column() {
		ScreenHeader(title = stringResource(R.string.keys_backup_screen_title), onBack = onBack)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(1f),
		) {
			Text(
				text = stringResource(R.string.keys_backup_screen_subtitle),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier.padding(horizontal = 24.dp, vertical = 8.dp)
			)
			seeds.forEach {
				SeedItem(seedName = it) {
					onSelected(it)
				}
			}
		}
	}
}



@Composable
private fun SeedItem(seedName: String, onClick: Callback) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = MaterialTheme.colors.fill6,
		modifier = Modifier
			.clickable(onClick = onClick)
			.padding(horizontal = 8.dp, vertical = 4.dp),
	) {
		Row(verticalAlignment = Alignment.CenterVertically) {
			Text(
				text = seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier
					.weight(1f)
					.padding(horizontal = 16.dp, vertical = 16.dp),
			)
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textSecondary),
				modifier = Modifier
					.padding(end = 8.dp)
					.size(28.dp)
			)
		}
	}
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
private fun PreviewBackupScreen() {
	SignerNewTheme {
		SeedBackupScreen(
			listOf("Seed", "Seed Some", "Another name"),
			coreNavController = rememberNavController(),
			{},
		) { some ->
		}
	}
}
