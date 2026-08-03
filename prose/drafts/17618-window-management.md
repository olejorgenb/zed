2025-08-15

Yes, please!

I'm kinda disappointed that the same limitation which Electron have/had(?) has "carried over" to Zed :/  My understanding is that in Electron-based editors this was (at least originally) a technical limitation?

Though I do see that adding multiple windows can create some UI complexities:

1. Do each window have dedicated sidebars/docks?
   - If not - what happens if the user invoke actions involving the docs?
   - If they do - perhaps it complicates some code which can currently assume there's only one instance of the panes
2. ...  (I can't think of any other right away)

I'm personally enticed by the model OP suggest - where any pane _can_ be popped out as a separate window (like in the JetBrains IDEs/GIMP). Though I have to admit I never used the functionality that much in JetBrains.

At the same time I think having multiple full-blown windows would be nice. Sometimes you want to explore paths in parallel, and then a dedicated window work better than eg. splits. [2].


[1]  Which is kinda in conflict with the pop-out pane model

[2] I suppose the [workaround](https://github.com/zed-industries/zed/discussions/12074#discussioncomment-12472029) using `zed -n` fits this scenario well though.



-----

2025-08-15 11:32

Mental model

1. Entities (which can be somewhat fuzzy)
  - Entities have identity
  - Entities are related to other entities
2. Actions / procedures
3. Stories?
4. Time

- The UI is an view into the user's mental model (ideally)
  - It allows the user to interact with it's model (ideally)

In practice it works differently:

- The UI is a view into an external model of the domain
  - It allow the user to interact with that model
  - It is a way of managing screen real estate

There will often/always be some mismatch between the user's mental model and this external model.

Flexible UIs allow the user to somewhat align these models.
But flexible UIs can be more confusing since they likely presents a less clear model to the user. So if the user does not have a clear model themself, it becomes very confusing.

Flexible UIs WILL necessarily carry some extra complexity (?).

With such UIs, finding the correct "block" and "connections" (concepts) is  extremely important.

A non-flexible UI guide the user, making the *specific* tasks the UI was designed for easy to perform.

Can we have the best of both worlds?

Yes? If we implement the inflexible UI on top of an flexible model?

Example:

- Make commands a first-class citizen and expose ways to work with them.

---

Limitations of UI:

1. Screen area
2. Colors (too many colors is just confusing)
3. Input sources

---

Deep interest: Can we find a very general model and UI for mental processes which is still useful?

And will such a model work across people?

A first step must be to find one which work for you!

It's all about attention?
Attention over time.
Expanding/augmenting memory
Working memory primarily
A way of persisting working memory - speeding up the process of context switching - which normally involves reconstructing the working memory from long term memory. (?)

A language for thinking. (?)

With an "IDE".

Many have tried... it's not exactly an original thought. "PKMS".

I'm not that impressesed so far. To be fair - I haven't tried them all, and perhaps not given the ones I've tried a decent chance (?)

Expressing nuances, not just extremes.

Learn from history?

Mind maps:

Q: what is the size/complexity limitation of planar graphs in relation to human understanding
  Meaning - when does a graph go from being obvious (assoc: subitize) to being just confusing and impossible to follow?

---

Short term memory is very limited.
Working memory is kinda bigger?
Long term memory is big, but slow (meaning it require kind of reconstruction to move into working memory)


---

# Elucidating the tab-model of Obsidian

Tabs are distinct viewports into the database.
Each tab have it's own history of the viewport content.
Multiple tabs can be open.
By default interactions operate within the active tab (or last focused tab). Eg.: navigation change the viewport content of the active tab (as opposed to opening a new one). This also hold when clicking on documents in the sidebar ("index") or from pickers.

Meaning that the semantic of "opening"/"activating"/"focusing" an element is not to "open it and giving it a spatial position", but rather to change the focus of the active tab to that element.

Splits is an orthogonal feature to tabs.

I think it would make sense to also allow splitting a tab's viewport such that it shows the tail of the viewport history. (assoc: )
