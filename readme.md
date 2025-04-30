# Aquarius Formal Verification Contest Details
- Total Prize Pool: $20,000 in USDC 
  - Real Bug Rules: $4,000 in USDC
  - Coverage Rules: $14,000 in USDC
  - Participation Rules: $2,000 in USDC
- Starts February 24, 2025 20:00 UTC
- Ends March 17, 2025 20:00 UTC
- This contest is part of the [Aquarius Audit + Certora Formal Verification competition](https://github.com/Certora/aquarius-cantina-fv)

## Introduction

The Formal Verification (FV) component of the contest is about using the Certora Sunbeam Prover to formally verify properties in the Soroban smart contracts in scope. Participants are incentivized to implement and verify high coverage properties. Submissions, incentives, and judging are different from the main contest so please read this document in its entirety.

Note that some setup work and basic properties are already being provided by the Certora Team. The mutations used for evaluation will not be caught by the existing properties.

## Scope

| Rust files | 
| --- |
| All files in the [fees_collector](https://github.com/Certora/aquarius-cantina-fv/tree/master/fees_collector) crate |
| All files in the [access_control](https://github.com/Certora/aquarius-cantina-fv/tree/master/access_control) crate |



## Overview
- 20,000 USDC of this contest will be allocated for FV.
- Conventional bug submission, issue judgment, and all reward distribution will be managed by Code4rena.
- FV component is unique as participants are incentivized to implement and verify high coverage properties using the Certora Prover.
- The judging of FV is conducted by Certora, with different submissions, incentives, and judging processes compared to the standard contest. These processes are explained in this document.

## Getting Started
- **Get access to the Prover**:
  - First time participants, [Register](https://www.certora.com/signup?plan=prover) to automatically receive an access key.
- **Update expired key**: 
  - Send a message in the [Certora Discord](https://discord.gg/certora)'s `access-key-request` channel.
- **Tool Installation**: 
  - Follow [installation instructions](https://docs.certora.com/en/latest/docs/sunbeam/installation.html) to download `certora-cli`. Use the latest version of the tool available at the start of the contest, throughout the whole contest.
- **Learning Resources**: 
  - Complete the [tutorials](https://certora-sunbeam-tutorials.readthedocs-hosted.com/en/latest/).
  - See [below](https://github.com/Certora/soroban-amm-contest-prep/blob/contest-prep/CONTEST.md#notes-and-tips) in this doc for additional help.
  - Search the [docs](https://docs.certora.com/en/latest/docs/sunbeam/index.html) for any additional information.
- **Contest Participation**:
  - [Import](https://github.com/new/import) this repository into a new private repository at the contest's commencement.
  - Conduct verifications on the master branch.
    - You can work in a separate branch and merge changes at the end if you prefer.  
  - Grant access to `teryanarmen` and `nd-certora` for judging.
- **Support Channels**:
  - For tool-related issues, send a detailed message with a job link in `help-desk` channel in Discord. Remove the anonymousKey component from the link if you wish to limit viewing to Certora employees. 
  - For FV contest questions, use the relevant community verification channel in Discord.
- **Certora folder**:
  - Certora infrastructure is made up of 5 folders listed below.
    - `fees_collector/confs`: configuration files for the tool. One example file is provided. More can be added if needed.
    - `fees_collector/src/certora_specs`: any specs you write should go here. You can add as many files as needed.
    - `fees_collector/src/certora_specs/summaries`: functions that need to be summarized for verification.
    - `fees_collector/src/certora_specs/mocks`: mock implementations needed for verification.
    - `fees_collector/mutations`: mutants folder which will be used to evaluate specs.
- **Compiling**:
  - To compile you code, run `just build` from the `fees_collector` directory.
  - To run the prover with some `conf` file do the following:
  ```
  cd confs
  certoraSorobanProver FILENAME.conf
  ```
  - Note that you might first need to run `chmod +x certora_build.py` before you run `cd confs`

## Incentives
- 20,000k of the total pool is allocated for FV.
- FV pool is split into three categories
  - **Participation**: 10% of pool awarded for properties identifying public mutants.    
  - **Real Bugs**: 20% of pool awarded for properties uncovering actual bugs.
  - **Coverage**: 70% of pool awarded for properties identifying private mutants.
- If no properties are accepted for real bugs, the pool will be rebalanced to 90% coverage and 10% participation.
- Mutants are mutated versions of the original code which create vulnerabilities. These mutants are used to gauge verified properties' coverage of the original code.
  - Public mutants used for evaluating participation rewards can be found in `fees_collector/mutations`.
- Participation and coverage reward can be calculated as follows
    - Each mutant is worth $0.9^{n-1}$ points where $n$ is the number of participants that caught the mutant.
    - If we let $P$ be the total FV pool and $T$ be the sum of all mutants' points, we can define each participant's reward as $ \frac{P}{T} \cdot \frac{0.9^{n-1}}{n} $
- Real bug rewards will be awarded for properties that are violated because of the bug. Only the bug submitter can submit a spec for that bug. 10, 3, or 1 points will be allocated based on the severity of the bug (H/M/L).

## Submission Guidelines
- **Submission**: 
  - Submit your work by sharing the private repo you cloned with `teryanarmen` and `nd-certora` on github.
  - Properties for real bugs will be submitted as github issues on the same private repo and must contain a link to the normal bug submission through Code4rena marked with relevant severity (L/M/H).
  - Submissions will not be public and will only be shared with the committee by sharing your private repo on github.

- **Team Participation**:
  - Working as a team is allowed and encouraged.
  - In case of solo catching/finding, team submissions will earn more than individual submissions.
  - Multiple submissions of the same work by different team members are not allowed and may result in submissions being disqualified.

- **Development Constraints**:
  - Participants are allowed to create and modify configuration, harnesses, and specification files.
    - Some conf files have commented out settings which can be used to help with running time.
  - All coverage and participation submissions must pass on the unaltered original codebase.
  - Source code modifications are prohibited.
    - Evaluations are based on the original code; configurations reliant on code changes will be disregarded.
  - Utilize the latest version of `certoraRun` available at contest start.
    - Make sure to update to the latest version of `certora-cli` before starting verification by running `pip install certora-cli --upgrade`.
    - Avoid updates during the contest, even if new versions are released.
    - Only update if explicitly told to by Certora team.
  - Submissions with tool errors, compilation errors, or timing-out rules will not be considered.
    - Ensure configurations do not time out; retest to confirm consistency.
- **Configuration Naming**:
  - For coverage and participation: Name configuration files as `ContractName_[identifier]_verified.conf`. The identifier is optional and should be used when multiple configurations are created for one contract.     
    - Example: `fee_collector_init_admin_verified.conf`.
  - For real bugs: Replace `_verified` with `_violated` in the configuration file name.
- **Rule Quality**:
  - Certora reserves the right to review and disqualify mutants which are only caught by low-quality rules.
  - Focus on creating valuable and secure rules that can potentially be added to the protocol CI.
  - Avoid submitting rules that simply copy the contract's code or provide little value in terms of security verification.
- **Real bug submissions**:
  - Real bug submissions must include:
    - A link to the  accepted underlying issue submitted through Code4rena.
    - Explaination of the property that finds the bug.
    - A link to a violated run of the property must be included.
    - A proposed solution as a diff between the buggy and fixed code.
    - A verified run of the property on the fixed version must be included.

## How to reference bugs submitted to [the Code4rena audit](XXX) 
1. On [your `Findings` screen](https://code4rena.com/findings), locate the submission you want.
1. Right-click on the link and select `Copy link address`.
1. Paste the URL into the related Github issue on your private clone which describes the rule they have for the bug. The URL structure should look similar to this: `XXX` 
   - Alternately, you can just include the UID at the end of the URL string (e.g. `XXX`).


## Evaluation Process
- **Preliminary Results**: Initial findings will be announced along with the mutations used for evaluation. A google sheet showing which mutants were caught by which participants will be shared. Participants will have a 72-hour period for review and submit corrections in case a certain mutant is marked as not caught but they actually caught it.
- **Correction Submissions**: Corrections must include a verified run on the source code and a violated run on the mutant. Any changes other than the mutation will result in exclusion of the correction.
- **Check your work**: Copy the mutants from `fees_collector/mutations` one at a time to the relevant directory and check that your spec catches them. You can search for the word "MUTANT" in each file to see the change.
    - Mutants `contract_0.rs` and `contract_1.rs` should be used to replace `fees_collector/src/contract.rs`.
    - Mutant `emergency_0.rs` should be used to replace `access_control/src/emergency.rs`.
- **Mutant Removal**: Certora reserves the right to remove any mutants that are caught only by low-value rules. This ensures that participants focus on creating valuable and secure rules rather than just catching mutations.
- **Reward Distribution**: If both low-quality and high-quality rules catch a mutation, rewards will be distributed normally.

## Report Compilation
- **Public Disclosure**: The report, encompassing top submissions and mutation descriptions, will be made public post-analysis.
  - Not all top properties focus on the quantity of mutations caught; high-level invariants are highly valued.
    - Future mutations will be adjusted to properly value high quality properties.
  - Guidelines for superior specifications:
    - Avoid code duplication.
    - Eschew simplistic unit tests.
    - Limit excessive assertions.
    - Focus on concise, high-level properties.
    - Reduce overuse of `cvlr_assume!` statements.
    - Ensure clear documentation, proper naming, and formatting.
- **Participant Contributions**: The top participants' `certora` folders will be included in the public repository.

## Notes and Tips
- We have provided a couple of example rules to help get started: `fees_collector/src/certora_specs/fee_collector_rules.rs`
- Notice the use of `clog!` for logging different values. You can use it for signed and unsigned integers (32, 64, 128 bit), and `Address` (see usage for `Address` in the rule `init_admin_sets_admin`).
- You can see how to create ghost variables by looking at the variable `ACCESS_CONTROL` and its updates.
- You can see the spec language we used [here](https://github.com/Certora/cvlr) and [here](https://github.com/Certora/cvlr-soroban).
- For debugging, we recommend running one rule at a time by changing the `rule` field in the `conf` files.
