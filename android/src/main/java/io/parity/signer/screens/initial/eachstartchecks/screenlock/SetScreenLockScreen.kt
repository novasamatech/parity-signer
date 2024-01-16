package io.parity.signer.screens.initial.eachstartchecks.screenlock

import android.content.Intent
import android.content.res.Configuration
import android.provider.Settings
import timber.log.Timber
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Password
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.app.ActivityCompat.startActivityForResult
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.domain.findActivity
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink500
import io.parity.signer.ui.theme.textTertiary


@Composable
fun SetScreenLockScreen() {
	Column(modifier = Modifier.padding(24.dp)) {
		Spacer(Modifier.weight(0.5f))
		Image(
			imageVector = Icons.Outlined.Password,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.pink500),
			modifier = Modifier
                .padding(horizontal = 8.dp)
                .size(80.dp)
                .align(Alignment.CenterHorizontally)
		)
		Spacer(modifier = Modifier.padding(top = 32.dp))
		Text(
			modifier = Modifier
                .fillMaxWidth(1f)
                .padding(horizontal = 8.dp),
			text = stringResource(R.string.screen_lock_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.padding(top = 16.dp))
		Text(
			modifier = Modifier
                .fillMaxWidth(1f)
                .padding(horizontal = 8.dp),
			text = stringResource(R.string.screen_lock_description),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.padding(top = 40.dp))
		val activity = LocalContext.current.findActivity()
		PrimaryButtonWide(label = stringResource(R.string.screen_lock_open_settings_button)) {
			val intent = Intent(Settings.ACTION_SECURITY_SETTINGS)
			if (intent.resolveActivity(activity!!.packageManager) != null) {
				startActivityForResult(
					activity,
					intent,
					OPEN_SCREEN_LOCK_SETTINGS_REQUEST_CODE,
					null
				)
			} else {
				val generalIntent = Intent(Settings.ACTION_SETTINGS)
				if (generalIntent.resolveActivity(activity.packageManager) != null) {
					startActivityForResult(
						activity,
						generalIntent,
						OPEN_SCREEN_LOCK_SETTINGS_REQUEST_CODE,
						null
					)
				} else {
					Timber.e("screen lock", "Settings activity not found")
				}
			}
		}
		Spacer(Modifier.weight(0.5f))
	}
}

private const val OPEN_SCREEN_LOCK_SETTINGS_REQUEST_CODE = 42


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewSetScreenLockScreen() {
	SignerNewTheme() {
		SetScreenLockScreen()
	}
}
