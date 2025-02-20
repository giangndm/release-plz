import React from 'react';

import type {Props as Tweet} from '../components/Tweet';

export type TweetItem = Tweet & {
  showOnHomepage: boolean;
};

const TWEETS: TweetItem[] = [
  {
    url: 'https://fosdem.org/2024/schedule/event/fosdem-2024-2682-semver-in-the-rust-ecosystem-breakage-tooling-and-edge-cases/',
    handle: 'PredragGruevski',
    name: 'Predrag Gruevski',
    date: 'Feb 3, 2024',
    content: (
      <>
        Release-plz will automatically run cargo-semver-checks as part of the release process. If you are in the market for a good release manager, you should check this one out; it's awesome.
      </>
    ),
    showOnHomepage: true,
    githubUsername: 'obi1kenobi',
  },
  {
    url: 'https://github.com/rust-lang/libc/issues/3350#issuecomment-1746436699',
    handle: 'fasterthanlime',
    name: 'Amos Wenger',
    date: 'Oct 4, 2023',
    content: (
      <>
        As far as I can tell, release-plz (an actual Rust executable running in CI, comes with its own GitHub Action) is best-of-class.
      </>
    ),
    showOnHomepage: true,
    githubUsername: 'fasterthanlime',
  },
  {
    url: 'https://www.reddit.com/r/rust/comments/13he55f/comment/jkckcx9/?utm_source=share&utm_medium=web2x&context=3',
    handle: 'XAMPPRocky',
    name: 'Livia Prima',
    date: 'May 16, 2023',
    content: (
      <>
        This is a great project to contribute to, it's so incredibly valuable for other maintainers like myself to be able to automate project releases and provide a better experience for our contributors. I'm already using it in octocrab and plan to adopt it in all my projects eventually and going forward.
      </>
    ),
    showOnHomepage: true,
    githubUsername: 'XAMPPRocky',
  },
];

export default TWEETS;
