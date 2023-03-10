package io.parity.signer.components.networkicon

import android.annotation.SuppressLint
import android.content.Context
import androidx.compose.runtime.Composable


@Composable
fun NetworkIcon(networkName: String) {

}

@SuppressLint("DiscouragedApi")
private fun Context.getIconForNetwork(networkName: String): Int? {
	return resources.getIdentifier(/* name = */ "network_$networkName",
		/* defType = */
		"drawable",
		/* defPackage = */
		packageName
	)
}
