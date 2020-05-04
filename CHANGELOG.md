# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)

## _[Unreleased]_

- docs: track project changes in CHANGELOG.md
- docs: improve macOS installation instructions
- docs: add README.md

[unreleased]: https://github.com/rustic-games/vienna/commits

<!--
Config(
  github: ( repo: "rustic-games/vienna" )
)

Template(
# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)

{%- for release in releases %}
- [`{{ release.version }}`](#{{ release.version | replace(from=".", to="") }}) – _{{ release.date | date(format="%Y.%m.%d")}}_
{%- endfor %}

## _[Unreleased]_

{% if unreleased.changes -%}
{%- for change in unreleased.changes -%}
- {{ change.type }}: {{ change.description }}
{% endfor %}
{% else -%}
_nothing new to show for… yet!_

{% endif -%}
{%- for release in releases -%}
## [{{ release.version }}]{% if release.title %} – _{{ release.title }}_{% endif %}

_{{ release.date | date(format="%Y.%m.%d") }}_
{%- if release.notes %}

{{ release.notes }}
{% endif -%}
{%- if release.changeset.contributors %}

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖
{% for contributor in release.changeset.contributors %}
- {{ contributor.name }} (<{{ contributor.email }}>)
{%- endfor %}
{%- endif %}

### Changes

{% for type, changes in release.changeset.changes | group_by(attribute="type") -%}

#### {{ type | typeheader }}

{% for change in changes -%}
- **{{ change.description }}** ([`{{ change.commit.short_id }}`])

{% if change.body -%}
{{ change.body | indent(n=2) }}

{% endif -%}
{%- endfor -%}

{% endfor %}
{%- endfor -%}

{% if config.github.repo -%}
  {%- set url = "https://github.com/" ~ config.github.repo -%}
{%- else -%}
  {%- set url = "#" -%}
{%- endif -%}
{% if releases -%}
[unreleased]: {{ url }}/compare/v{{ releases | first | get(key="version") }}...HEAD
{%- else -%}
[unreleased]: {{ url }}/commits
{%- endif -%}
{%- for release in releases %}
[{{ release.version }}]: {{ url }}/releases/tag/v{{ release.version }}
{%- endfor %}

{%- for release in releases %}
{%- for change in release.changeset.changes %}
[`{{ change.commit.short_id }}`]: {{ url }}/commit/{{ change.commit.id }}
{%- endfor -%}
{%- endfor %}

)
-->
