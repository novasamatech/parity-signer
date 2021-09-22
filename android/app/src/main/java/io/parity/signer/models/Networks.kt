package io.parity.signer.models

/**
 * This is how hard types should look like; just copy from ios I suppose?
 */
data class Network(
	val key: String,
	val color: String,
	val logo: String,
	val order: String,
	val secondaryColor: String,
	val title: String
)
