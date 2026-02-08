/*
 * Layer state notifier via Raw HID
 *
 * Sends layer state changes to the host PC using zmk-raw-hid,
 * equivalent to QMK's layer_state_set_user() callback.
 *
 * Packet format (matches QMK convention):
 *   data[0]   = 0xFF (marker byte)
 *   data[1]   = sizeof(uint32_t) = 4
 *   data[2-5] = default layer state bitmask (little-endian)
 *   data[6-9] = active layer state bitmask (little-endian)
 */

#include <zmk/event_manager.h>
#include <zmk/events/layer_state_changed.h>
#include <zmk/keymap.h>

#include <raw_hid/events.h>

#include <string.h>

#include <zephyr/logging/log.h>
LOG_MODULE_DECLARE(zmk, CONFIG_ZMK_LOG_LEVEL);

#define LAYER_NOTIFY_MARKER 0xFF
#define MAX_LAYER_CHECK 32

static uint8_t layer_hid_buf[CONFIG_RAW_HID_REPORT_SIZE];

static int layer_state_changed_listener(const zmk_event_t *eh) {
    /* Build active-layer bitmask (equivalent to QMK layer_state_t) */
    uint32_t layer_state = 0;
    for (uint8_t i = 0; i < MAX_LAYER_CHECK; i++) {
        if (zmk_keymap_layer_active(i)) {
            layer_state |= BIT(i);
        }
    }

    /* Default layer is always layer 0 in ZMK */
    uint32_t default_layer_state = BIT(0);

    memset(layer_hid_buf, 0, sizeof(layer_hid_buf));
    layer_hid_buf[0] = LAYER_NOTIFY_MARKER;       /* 0xFF marker           */
    layer_hid_buf[1] = sizeof(uint32_t);           /* size of layer state   */
    memcpy(&layer_hid_buf[2], &default_layer_state, sizeof(uint32_t));
    memcpy(&layer_hid_buf[2 + sizeof(uint32_t)], &layer_state, sizeof(uint32_t));

    raise_raw_hid_sent_event(
        (struct raw_hid_sent_event){.data = layer_hid_buf, .length = sizeof(layer_hid_buf)});

    return ZMK_EV_EVENT_BUBBLE;
}

ZMK_LISTENER(layer_notifier, layer_state_changed_listener);
ZMK_SUBSCRIPTION(layer_notifier, zmk_layer_state_changed);
