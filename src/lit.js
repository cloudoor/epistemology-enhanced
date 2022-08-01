
/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const t = globalThis,
  s =
    t.ShadowRoot &&
    (void 0 === t.ShadyCSS || t.ShadyCSS.nativeShadow) &&
    "adoptedStyleSheets" in Document.prototype &&
    "replace" in CSSStyleSheet.prototype,
  i = Symbol(),
  e = new WeakMap();
class o {
  constructor(t, s, e) {
    if (((this._$cssResult$ = !0), e !== i))
      throw Error(
        "CSSResult is not constructable. Use `unsafeCSS` or `css` instead."
      );
    (this.cssText = t), (this.t = s);
  }
  get styleSheet() {
    let t = this.i;
    const i = this.t;
    if (s && void 0 === t) {
      const s = void 0 !== i && 1 === i.length;
      s && (t = e.get(i)),
        void 0 === t &&
          ((this.i = t = new CSSStyleSheet()).replaceSync(this.cssText),
          s && e.set(i, t));
    }
    return t;
  }
  toString() {
    return this.cssText;
  }
}
const h = (t) => new o("string" == typeof t ? t : t + "", void 0, i),
  r = (t, ...s) => {
    const e =
      1 === t.length
        ? t[0]
        : s.reduce(
            (s, i, e) =>
              s +
              ((t) => {
                if (!0 === t._$cssResult$) return t.cssText;
                if ("number" == typeof t) return t;
                throw Error(
                  "Value passed to 'css' function must be a 'css' function result: " +
                    t +
                    ". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security."
                );
              })(i) +
              t[e + 1],
            t[0]
          );
    return new o(e, t, i);
  },
  n = (i, e) => {
    if (s)
      i.adoptedStyleSheets = e.map((t) =>
        t instanceof CSSStyleSheet ? t : t.styleSheet
      );
    else
      for (const s of e) {
        const e = document.createElement("style"),
          o = t.litNonce;
        void 0 !== o && e.setAttribute("nonce", o),
          (e.textContent = s.cssText),
          i.appendChild(e);
      }
  },
  c = s
    ? (t) => t
    : (t) =>
        t instanceof CSSStyleSheet
          ? ((t) => {
              let s = "";
              for (const i of t.cssRules) s += i.cssText;
              return h(s);
            })(t)
          : t,
  /**
   * @license
   * Copyright 2017 Google LLC
   * SPDX-License-Identifier: BSD-3-Clause
   */ {
    is: a,
    defineProperty: l,
    getOwnPropertyDescriptor: u,
    getOwnPropertyNames: d,
    getOwnPropertySymbols: f,
    getPrototypeOf: p,
  } = Object,
  v = globalThis,
  m = v.trustedTypes,
  y = m ? m.emptyScript : "",
  g = v.reactiveElementPolyfillSupport,
  _ = (t, s) => t,
  b = {
    toAttribute(t, s) {
      switch (s) {
        case Boolean:
          t = t ? y : null;
          break;
        case Object:
        case Array:
          t = null == t ? t : JSON.stringify(t);
      }
      return t;
    },
    fromAttribute(t, s) {
      let i = t;
      switch (s) {
        case Boolean:
          i = null !== t;
          break;
        case Number:
          i = null === t ? null : Number(t);
          break;
        case Object:
        case Array:
          try {
            i = JSON.parse(t);
          } catch (t) {
            i = null;
          }
      }
      return i;
    },
  },
  S = (t, s) => !a(t, s),
  w = { attribute: !0, type: String, converter: b, reflect: !1, hasChanged: S };
(Symbol.metadata ??= Symbol("metadata")),
  (v.litPropertyMetadata ??= new WeakMap());
class $ extends HTMLElement {
  static addInitializer(t) {
    this.o(), (this.l ??= []).push(t);
  }
  static get observedAttributes() {
    return this.finalize(), this.u && [...this.u.keys()];
  }
  static createProperty(t, s = w) {
    if (
      (s.state && (s.attribute = !1),
      this.o(),
      this.elementProperties.set(t, s),
      !s.noAccessor)
    ) {
      const i = Symbol(),
        e = this.getPropertyDescriptor(t, i, s);
      void 0 !== e && l(this.prototype, t, e);
    }
  }
  static getPropertyDescriptor(t, s, i) {
    const { get: e, set: o } = u(this.prototype, t) ?? {
      get() {
        return this[s];
      },
      set(t) {
        this[s] = t;
      },
    };
    return {
      get() {
        return e?.call(this);
      },
      set(s) {
        const h = e?.call(this);
        o.call(this, s), this.requestUpdate(t, h, i);
      },
      configurable: !0,
      enumerable: !0,
    };
  }
  static getPropertyOptions(t) {
    return this.elementProperties.get(t) ?? w;
  }
  static o() {
    if (this.hasOwnProperty(_("elementProperties"))) return;
    const t = p(this);
    t.finalize(),
      void 0 !== t.l && (this.l = [...t.l]),
      (this.elementProperties = new Map(t.elementProperties));
  }
  static finalize() {
    if (this.hasOwnProperty(_("finalized"))) return;
    if (
      ((this.finalized = !0), this.o(), this.hasOwnProperty(_("properties")))
    ) {
      const t = this.properties,
        s = [...d(t), ...f(t)];
      for (const i of s) this.createProperty(i, t[i]);
    }
    const t = this[Symbol.metadata];
    if (null !== t) {
      const s = litPropertyMetadata.get(t);
      if (void 0 !== s)
        for (const [t, i] of s) this.elementProperties.set(t, i);
    }
    this.u = new Map();
    for (const [t, s] of this.elementProperties) {
      const i = this.p(t, s);
      void 0 !== i && this.u.set(i, t);
    }
    this.elementStyles = this.finalizeStyles(this.styles);
  }
  static finalizeStyles(t) {
    const s = [];
    if (Array.isArray(t)) {
      const i = new Set(t.flat(1 / 0).reverse());
      for (const t of i) s.unshift(c(t));
    } else void 0 !== t && s.push(c(t));
    return s;
  }
  static p(t, s) {
    const i = s.attribute;
    return !1 === i
      ? void 0
      : "string" == typeof i
      ? i
      : "string" == typeof t
      ? t.toLowerCase()
      : void 0;
  }
  constructor() {
    super(),
      (this.v = void 0),
      (this.isUpdatePending = !1),
      (this.hasUpdated = !1),
      (this.m = null),
      this._();
  }
  _() {
    (this.S = new Promise((t) => (this.enableUpdating = t))),
      (this._$AL = new Map()),
      this.$(),
      this.requestUpdate(),
      this.constructor.l?.forEach((t) => t(this));
  }
  addController(t) {
    (this.P ??= new Set()).add(t),
      void 0 !== this.renderRoot && this.isConnected && t.hostConnected?.();
  }
  removeController(t) {
    this.P?.delete(t);
  }
  $() {
    const t = new Map(),
      s = this.constructor.elementProperties;
    for (const i of s.keys())
      this.hasOwnProperty(i) && (t.set(i, this[i]), delete this[i]);
    t.size > 0 && (this.v = t);
  }
  createRenderRoot() {
    const t =
      this.shadowRoot ?? this.attachShadow(this.constructor.shadowRootOptions);
    return n(t, this.constructor.elementStyles), t;
  }
  connectedCallback() {
    (this.renderRoot ??= this.createRenderRoot()),
      this.enableUpdating(!0),
      this.P?.forEach((t) => t.hostConnected?.());
  }
  enableUpdating(t) {}
  disconnectedCallback() {
    this.P?.forEach((t) => t.hostDisconnected?.());
  }
  attributeChangedCallback(t, s, i) {
    this._$AK(t, i);
  }
  C(t, s) {
    const i = this.constructor.elementProperties.get(t),
      e = this.constructor.p(t, i);
    if (void 0 !== e && !0 === i.reflect) {
      const o = (
        void 0 !== i.converter?.toAttribute ? i.converter : b
      ).toAttribute(s, i.type);
      (this.m = t),
        null == o ? this.removeAttribute(e) : this.setAttribute(e, o),
        (this.m = null);
    }
  }
  _$AK(t, s) {
    const i = this.constructor,
      e = i.u.get(t);
    if (void 0 !== e && this.m !== e) {
      const t = i.getPropertyOptions(e),
        o =
          "function" == typeof t.converter
            ? { fromAttribute: t.converter }
            : void 0 !== t.converter?.fromAttribute
            ? t.converter
            : b;
      (this.m = e), (this[e] = o.fromAttribute(s, t.type)), (this.m = null);
    }
  }
  requestUpdate(t, s, i) {
    if (void 0 !== t) {
      if (
        ((i ??= this.constructor.getPropertyOptions(t)),
        !(i.hasChanged ?? S)(this[t], s))
      )
        return;
      this.T(t, s, i);
    }
    !1 === this.isUpdatePending && (this.S = this.A());
  }
  T(t, s, i) {
    this._$AL.has(t) || this._$AL.set(t, s),
      !0 === i.reflect && this.m !== t && (this.M ??= new Set()).add(t);
  }
  async A() {
    this.isUpdatePending = !0;
    try {
      await this.S;
    } catch (t) {
      Promise.reject(t);
    }
    const t = this.scheduleUpdate();
    return null != t && (await t), !this.isUpdatePending;
  }
  scheduleUpdate() {
    return this.performUpdate();
  }
  performUpdate() {
    if (!this.isUpdatePending) return;
    if (!this.hasUpdated) {
      if (((this.renderRoot ??= this.createRenderRoot()), this.v)) {
        for (const [t, s] of this.v) this[t] = s;
        this.v = void 0;
      }
      const t = this.constructor.elementProperties;
      if (t.size > 0)
        for (const [s, i] of t)
          !0 !== i.wrapped ||
            this._$AL.has(s) ||
            void 0 === this[s] ||
            this.T(s, this[s], i);
    }
    let t = !1;
    const s = this._$AL;
    try {
      (t = this.shouldUpdate(s)),
        t
          ? (this.willUpdate(s),
            this.P?.forEach((t) => t.hostUpdate?.()),
            this.update(s))
          : this.k();
    } catch (s) {
      throw ((t = !1), this.k(), s);
    }
    t && this._$AE(s);
  }
  willUpdate(t) {}
  _$AE(t) {
    this.P?.forEach((t) => t.hostUpdated?.()),
      this.hasUpdated || ((this.hasUpdated = !0), this.firstUpdated(t)),
      this.updated(t);
  }
  k() {
    (this._$AL = new Map()), (this.isUpdatePending = !1);
  }
  get updateComplete() {
    return this.getUpdateComplete();
  }
  getUpdateComplete() {
    return this.S;
  }
  shouldUpdate(t) {
    return !0;
  }
  update(t) {
    (this.M &&= this.M.forEach((t) => this.C(t, this[t]))), this.k();
  }
  updated(t) {}
  firstUpdated(t) {}
}
($.elementStyles = []),
  ($.shadowRootOptions = { mode: "open" }),
  ($[_("elementProperties")] = new Map()),
  ($[_("finalized")] = new Map()),
  g?.({ ReactiveElement: $ }),
  (v.reactiveElementVersions ??= []).push("2.0.4");
/**
 * @license