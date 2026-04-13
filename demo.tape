# VHS demo tape for ratatui-form
# Run with: vhs demo.tape

Output demo.gif

Set Theme "Catppuccin Mocha"
Set FontSize 20
Set Width 1200
Set Height 600
Set WindowBar Colorful
Set TypingSpeed 50ms

# Build the example and clear the screen before recording starts
Hide
Type "cargo build --example address_form --release"
Enter
Wait
Type "clear"
Enter
Show

# Start the form
Type "./target/release/examples/address_form"
Enter
Sleep 1s

# Name
Type "Jane Doe"
Sleep 800ms
Tab
Sleep 500ms

# Email
Type "jane@example.com"
Sleep 800ms
Tab
Sleep 500ms

# Phone
Type "(555) 867-5309"
Sleep 800ms
Tab
Sleep 500ms

# Address street1
Type "742 Evergreen Terrace"
Sleep 800ms
Tab
Sleep 500ms

# Address street2
Type "Apt 3B"
Sleep 800ms
Tab
Sleep 500ms

# City
Type "Springfield"
Sleep 800ms
Tab
Sleep 500ms

# State dropdown
Space
Sleep 800ms
Down@80ms 10
Sleep 500ms
Enter
Sleep 800ms
Tab
Sleep 500ms

# ZIP
Type "62701"
Sleep 800ms
Tab
Sleep 500ms

# Newsletter checkbox
Space
Sleep 600ms
Tab
Sleep 500ms

# Terms checkbox
Space
Sleep 800ms
Tab
Sleep 500ms

# Submit
Enter
Sleep 3s
