# Decreasing target OS version

Why I'm doing this? Just to scale up UX testing quickly! This will be undone and never mentioned again, we aren't talking about it, ok?

# Change build settings

Decreased one value in project, one in each used target

Build fails with many errors

# Focus state

It was introduced in 15.0

1. Find all `@FocusState` variables and comment them out. Find their use and comment it out. Or just delete it all, it's a dead end anyway. While I'm at it, comment out `.submitLabel` which is new as well. `prompt` field in `TextField` goes as well - this should be replaced later. `.onSubmit()` modifier should be replaced by `onCommit` callback (in form of `TextField(...) { yourcodehere }`

- `NewSeed`
- `NewAddressScreen`
- `TransactionReady` - here remember to keep onSubmit functionality

# AttributedString and TextField prompt

Markdown support did not exist properly prior to iOS 15.0 - they used some JS-powered WebView abomination for it. Maybe they still do it under the hood, who knows? But at least not it's tested, official and should not do anything funny.

1. Go to `Models/Utils` and change `extension AttributedString` into `extension String` so that next steps are simpler

2. Fix the init function so that it just unhexes string

3. Find all `AttributedString` inits in code and replace them with `String`

- `TCCall`
- `TCEnumVariantName`
- `TCFieldNumber`
- `TCFieldName`
- `Models/Documents`
- `TCPallet`
- `TransactionCommentInput`

4. Replace `ppMD` with `pp` and `taCMD` with `tac` in `Models/Documents`

# Woodoo

After all this, compiler fails with "Command MergeSwiftModule failed with a nonzero exit code"

This is building bug; clearing cache by command+shift+K fixed this issue

Hmm, at this moment everything seems fine. Let's commit this.
