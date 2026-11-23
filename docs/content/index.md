---
seo:
  title: Laks documentation
  description: Getting started, using the Laks interface, and understanding the
    current Python package registry implementation.
---

::warning
Laks is under active development. Package workflows are incomplete, and the
backend does not yet enforce authorization on user-management or registry
routes. Do not expose this build to untrusted networks.
::

::u-page-hero
#title
The self-hosted Python registry

#description
Laks is an early-stage project for managing private Python packages. Learn how
to run the service, explore the web interface, and see what is still in
development.

#links
  :::u-button
  ---
  color: neutral
  size: xl
  to: /getting-started/introduction
  trailing-icon: i-lucide-arrow-right
  ---
  Explore Laks
  :::
::

::u-page-section
#title
Start with the essentials

#features
  :::u-page-feature
  ---
  icon: i-lucide-terminal
  to: /getting-started/installation
  ---
  #title
  Run it locally

  #description
  Start the Rust backend, configure its first administrator, and run the web UI.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-layout-dashboard
  to: /essentials/dashboard
  ---
  #title
  Explore the interface

  #description
  See which screens are connected to the service and which are still placeholders.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-package
  to: /essentials/python-packages
  ---
  #title
  Check package support

  #description
  Understand what the initial registry routes support—and what is not ready yet.
  :::
::
