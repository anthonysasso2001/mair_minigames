<!-- omit in toc -->

# Contributing to MAIR Minigames

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution.

<!-- omit in toc -->

## Table of Contents

- [Contributing to MAIR Minigames](#contributing-to-mair-minigames)
  - [Table of Contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [I Have a Question](#i-have-a-question)
  - [I Want To Contribute](#i-want-to-contribute)
    - [Reporting Bugs](#reporting-bugs)
      - [Before Submitting a Bug Report](#before-submitting-a-bug-report)
      - [How Do I Submit a Good Bug Report?](#how-do-i-submit-a-good-bug-report)
    - [Suggesting Enhancements](#suggesting-enhancements)
      - [Before Submitting an Enhancement](#before-submitting-an-enhancement)
      - [How Do I Submit a Good Enhancement Suggestion?](#how-do-i-submit-a-good-enhancement-suggestion)
    - [Your First Code Contribution](#your-first-code-contribution)
    - [Improving The Documentation](#improving-the-documentation)
      - [functions or macros](#functions-or-macros)
  - [Styleguides](#styleguides)
    - [Commit Messages](#commit-messages)
  - [Join The Project Team](#join-the-project-team)
  - [Attribution](#attribution)

## Code of Conduct

This project and everyone participating in it is governed by the
[MAIR Minigames Code of Conduct](https://github.com/anthonysasso2001/mair_minigamesblob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <<!-- TODO: Add admin contact info here -->>.

## I Have a Question

> If you want to ask a question, we assume that you have read the available [Documentation](https://github.com/anthonysasso2001/mair_minigames/wiki).

Before you ask a question, it is best to search for existing [Issues](https://github.com/anthonysasso2001/mair_minigames/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/anthonysasso2001/mair_minigames/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->

#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://github.com/anthonysasso2001/mair_minigames/wiki). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/anthonysasso2001/mair_minigamesissues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->

#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to <<!-- TODO: Add admin info here-->>.

<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/anthonysasso2001/mair_minigames/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for MAIR Minigames, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->

#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://github.com/anthonysasso2001/mair_minigames/wiki) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/anthonysasso2001/mair_minigames/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->

#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/anthonysasso2001/mair_minigames/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most MAIR Minigames users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution

All code contributions must be made from a branch of the repo, which will then be merged upon review.

The requirements for a PR are listed in the [code request template file](pull_request_template.md) which will require that the details be fully complete before a review can occur.

The requires software for code contribution (outside of cargo/rust) is as follows:

- [taurpaulin](https://github.com/xd009642/tarpaulin).
- [cargo-audit](https://lib.rs/crates/cargo-audit).
- [commitizen](https://commitizen-tools.github.io/commitizen/)

### Improving The Documentation

All added functions **MUST** be documented, with the format for documentation being mainly sourced from the [rust by example tutorial](https://doc.rust-lang.org/rust-by-example/meta/doc.html). Inline comments should be in the form of `// Goal; Reason.`

#### functions or macros

**comment template:**

````rust
/// Description and output for function.
///
/// # Attributes
/// * `name` - description
///
/// # Example
/// ```
/// [example use code here when applicable]
/// ```
````

**example:**

````rust
/// Macro for generating lazy regex object with just regex string for initializing static regex object for checks
///
/// # Attributes
/// * `regStr` - input regex string in the form r"str"
///
/// # Example
/// ```
/// // Generates a regex string for number inputs; added to reduce complexity here with multiple regex object definitions
/// static NUM_RE: Lazy<Regex> = regex!(r"^([\d]+)$");
/// NUM_RE.is_match(in_str);
/// ```
[derive(Debug)] // here to show comments go above this define; this doesn't make sense for a macro but needed for this example.
 macro_rules! regex {
        ($regStr:expr) => {
            Lazy::new(|| Regex::new($regStr).unwrap())
        };
    }
````

## Styleguides

### Commit Messages

For this project, please use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/), as this is being used with [git-cliff](https://git-cliff.org/) for changelog generation so commits which do not fit this scheme will cause issues.

## Join The Project Team

<!-- TODO -->

<!-- omit in toc -->

## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
Additional inclusions and style considerations are based off [ratatui](https://github.com/ratatui-org/ratatui) a rust tui framework
