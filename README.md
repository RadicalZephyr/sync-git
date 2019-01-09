# Sync Git!

If you use git for version control, chances are pretty good that you
have more than one git repository. If you're like me, you have a _lot_
of git repositories on your computer at any one time. Git by it's
decentralized nature is a natural tool to use to safely store code in
multiple physical locations to prevent data loss. However, this only
works if the code is synced with at least one remote.  In an ideal
world code would always be synced to a git repo as it's written, but
there are all sorts of reasons why that might happen (private
exploratory branches, stashed changes for local development
purposes). Once you have unpushed changes in a repo that you aren't
actively working on, the status of their survivability becomes unknown
very quickly.  For the most part this doesn't matter, until you want
to transition to a new computer.

Personally, I haven't gotten rid of a computer in the last decade
because I've never been fully confident that erasing the hard disk
wouldn't cause me to lose some code.

Enter sync-git. It crawls a directory tree looking for git
repositories and then checks to make sure that all local content is
synced with at least one remote. It can compile a report of unsynced
content across all the repositories it finds. Or, in interactive mode
you can direct sync-git how to handle specific cases while it
continues crawling in the background. Or you can configure it to the
nth-degree and establish policy for all kinds of situations and let
sync-git handle it fully automatically.

# License

Copyright 2019, Geoff Shannon

This file is part of Sync Git.

Sync Git  is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Hermit is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Sync Git. If not, see <http://www.gnu.org/licenses/>.
