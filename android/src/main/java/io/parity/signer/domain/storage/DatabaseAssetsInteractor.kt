package io.parity.signer.domain.storage

import android.annotation.SuppressLint
import android.content.Context
import android.security.keystore.UserNotAuthenticatedException
import io.parity.signer.domain.getDbNameFromContext
import okhttp3.internal.wait
import java.io.File
import java.io.FileOutputStream


/**
 * All interactions should happen only when user is authorised
 */
class DatabaseAssetsInteractor(
	private val context: Context,
	private val seedStorage: SeedStorage,
	private val cryptedStorage: ClearCryptedStorage,
) {
	private val dbName: String = context.getDbNameFromContext()

	/**
	 * Wipes all data
	 * @throws UserNotAuthenticatedException
	 */
	@SuppressLint("ApplySharedPref")
	fun wipe() {
		deleteDir(File(dbName))
		seedStorage.wipe()
		cryptedStorage.wipe()
	}

	/**
	 * Util to copy Assets to data dir; only used in onBoard().
	 * @throws UserNotAuthenticatedException
	 */
	fun copyAsset(path: String) {
		val contents = context.assets.list("Database$path")
		if (contents == null || contents.isEmpty()) {
			copyFileAsset(path)
		} else {
			File(dbName, path).mkdirs()
			for (entry in contents) copyAsset("$path/$entry")
		}
	}

	/**
	 * Util to copy single Assets file
	 * @throws UserNotAuthenticatedException
	 */
	private fun copyFileAsset(path: String) {
		val file = File(dbName, path)
		file.createNewFile()
		val input = context.assets.open("Database$path")
		val output = FileOutputStream(file)
		val buffer = ByteArray(1024)
		var read = input.read(buffer)
		while (read != -1) {
			output.write(buffer, 0, read)
			read = input.read(buffer)
		}
		output.close()
		input.close()
	}

	/**
	 * Util to remove directory
	 * @throws UserNotAuthenticatedException
	 */
	private fun deleteDir(fileOrDirectory: File) {
		if (fileOrDirectory.isDirectory) {
			val listFiles = fileOrDirectory.listFiles()
			if (!listFiles.isNullOrEmpty()) {
				for (child in listFiles) deleteDir(child)
			}
		}
		fileOrDirectory.delete()
	}
}
