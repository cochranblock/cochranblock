/** Unlicense — public domain — cochranblock.org */
(function() {
  'use strict';

  var WEEKDAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  var MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];

  function parseSlots() {
    var el = document.getElementById('booking-slots-data');
    if (!el || !el.textContent) return [];
    try {
      return JSON.parse(el.textContent);
    } catch (e) {
      return [];
    }
  }

  function dateKey(year, month, day) {
    var m = String(month + 1);
    var d = String(day);
    if (m.length < 2) m = '0' + m;
    if (d.length < 2) d = '0' + d;
    return year + '-' + m + '-' + d;
  }

  function buildAvailableSet(slots) {
    var set = {};
    for (var i = 0; i < slots.length; i++) {
      set[slots[i].date] = slots[i];
    }
    return set;
  }

  function renderMonth(gridEl, year, month, available) {
    var first = new Date(year, month, 1);
    var last = new Date(year, month + 1, 0);
    var startDow = first.getDay();
    var daysInMonth = last.getDate();
    var today = new Date();
    var todayKey = dateKey(today.getFullYear(), today.getMonth(), today.getDate());

    gridEl.innerHTML = '';
    gridEl.setAttribute('aria-label', MONTHS[month] + ' ' + year);

    for (var d = 0; d < WEEKDAYS.length; d++) {
      var th = document.createElement('div');
      th.className = 'booking-calendar-weekday';
      th.setAttribute('role', 'columnheader');
      th.textContent = WEEKDAYS[d];
      gridEl.appendChild(th);
    }

    var pad = startDow;
    for (var p = 0; p < pad; p++) {
      var empty = document.createElement('div');
      empty.className = 'booking-calendar-day booking-calendar-day-empty';
      empty.setAttribute('aria-hidden', 'true');
      gridEl.appendChild(empty);
    }

    for (var day = 1; day <= daysInMonth; day++) {
      var key = dateKey(year, month, day);
      var slot = available[key];
      var cell = document.createElement('div');
      cell.className = 'booking-calendar-day';
      cell.setAttribute('role', 'gridcell');
      cell.setAttribute('data-date', key);

      var btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'booking-calendar-day-btn';
      btn.textContent = day;
      btn.setAttribute('aria-label', 'Select ' + MONTHS[month] + ' ' + day + (slot ? ', available' : ''));

      if (slot) {
        btn.classList.add('booking-calendar-day-available');
        btn.addEventListener('click', function(k, s) {
          return function() { selectDate(k, s); };
        }(key, slot));
      } else {
        btn.classList.add('booking-calendar-day-unavailable');
        btn.disabled = true;
      }

      if (key === todayKey) {
        btn.classList.add('booking-calendar-day-today');
      }

      cell.appendChild(btn);
      gridEl.appendChild(cell);
    }
  }

  function selectDate(dateKey, slot) {
    var panel = document.getElementById('booking-time-panel');
    var heading = document.getElementById('booking-time-heading');
    var slotsEl = document.getElementById('booking-time-slots');
    var gridEl = document.getElementById('booking-grid');

    if (gridEl) {
      var prev = gridEl.querySelector('.booking-calendar-day-selected');
      if (prev) prev.classList.remove('booking-calendar-day-selected');
      var cell = gridEl.querySelector('[data-date="' + dateKey + '"]');
      if (cell) cell.classList.add('booking-calendar-day-selected');
    }

    heading.textContent = 'Pick a time for ' + slot.day_name + ', ' + slot.date_label;
    slotsEl.innerHTML = '';

    for (var i = 0; i < slot.times.length; i++) {
      var t = slot.times[i];
      var b = document.createElement('button');
      b.type = 'button';
      b.className = 'booking-time-slot';
      b.textContent = t.label;
      b.setAttribute('role', 'button');
      b.addEventListener('click', function(d, s, label) {
        return function() { selectTime(d, s, label); };
      }(dateKey, slot, t.label));
      slotsEl.appendChild(b);
    }

    panel.hidden = false;
    panel.setAttribute('aria-hidden', 'false');
    panel.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    var firstSlot = slotsEl.querySelector('.booking-time-slot');
    if (firstSlot) firstSlot.focus();
  }

  function selectTime(dateKey, slot, timeLabel) {
    var slotsEl = document.getElementById('booking-time-slots');
    if (slotsEl) {
      var prev = slotsEl.querySelector('.booking-time-slot-selected');
      if (prev) prev.classList.remove('booking-time-slot-selected');
      var btns = slotsEl.querySelectorAll('.booking-time-slot');
      for (var i = 0; i < btns.length; i++) {
        if (btns[i].textContent === timeLabel) {
          btns[i].classList.add('booking-time-slot-selected');
          break;
        }
      }
    }
    var form = document.getElementById('booking-form');
    var dIso = document.getElementById('booking-form-date');
    var tIso = document.getElementById('booking-form-time');
    var when = document.getElementById('booking-form-when');
    if (dIso) dIso.value = dateKey;
    if (tIso) tIso.value = timeLabel;
    if (when) when.textContent = slot.day_name + ', ' + slot.date_label + ' at ' + timeLabel;
    if (form) {
      form.hidden = false;
      form.setAttribute('aria-hidden', 'false');
      form.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      var nameInput = document.getElementById('bk-name');
      if (nameInput) nameInput.focus();
    }
  }

  function init() {
    var slots = parseSlots();
    var available = buildAvailableSet(slots);
    var gridEl = document.getElementById('booking-grid');
    var monthEl = document.getElementById('booking-month');
    var prevBtn = document.getElementById('booking-prev');
    var nextBtn = document.getElementById('booking-next');

    if (!gridEl || !monthEl || !prevBtn || !nextBtn) return;

    var now = new Date();
    var viewYear = now.getFullYear();
    var viewMonth = now.getMonth();

    function countAvailableInMonth(year, month, available) {
      var count = 0;
      var last = new Date(year, month + 1, 0);
      for (var d = 1; d <= last.getDate(); d++) {
        var key = dateKey(year, month, d);
        if (available[key]) count++;
      }
      return count;
    }

    function update() {
      monthEl.textContent = MONTHS[viewMonth] + ' ' + viewYear;
      var badge = document.getElementById('booking-available-badge');
      if (badge) {
        var n = countAvailableInMonth(viewYear, viewMonth, available);
        badge.textContent = n > 0 ? n + ' available' : '';
        badge.className = 'booking-available-badge' + (n > 0 ? ' booking-available-badge-active' : '');
      }
      renderMonth(gridEl, viewYear, viewMonth, available);
    }

    prevBtn.addEventListener('click', function() {
      viewMonth--;
      if (viewMonth < 0) {
        viewMonth = 11;
        viewYear--;
      }
      update();
    });

    nextBtn.addEventListener('click', function() {
      viewMonth++;
      if (viewMonth > 11) {
        viewMonth = 0;
        viewYear++;
      }
      update();
    });

    update();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
