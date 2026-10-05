#import <AppKit/AppKit.h>
#import <CoreGraphics/CoreGraphics.h>
#import <Foundation/Foundation.h>

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    if (argc != 2) {
        fputs("usage: macos-window-state PID\n", stderr);
        return 2;
    }

    pid_t pid = (pid_t)strtol(argv[1], NULL, 10);
    NSArray<NSDictionary *> *windows = CFBridgingRelease(
        CGWindowListCopyWindowInfo(kCGWindowListOptionAll, kCGNullWindowID));
    BOOL visible = NO;
    for (NSDictionary *window in windows) {
        NSNumber *owner_pid = window[(__bridge NSString *)kCGWindowOwnerPID];
        NSNumber *layer = window[(__bridge NSString *)kCGWindowLayer];
        NSNumber *on_screen = window[(__bridge NSString *)kCGWindowIsOnscreen];
        if (owner_pid.intValue == pid && layer.intValue == 0 && on_screen.boolValue) {
            visible = YES;
            break;
        }
    }

    BOOL frontmost = NSWorkspace.sharedWorkspace.frontmostApplication.processIdentifier == pid;
    NSRunningApplication *application =
        [NSRunningApplication runningApplicationWithProcessIdentifier:pid];
    NSInteger activation_policy = application ? application.activationPolicy : -1;
    BOOL active = application.isActive;
    printf("visible=%s\nfrontmost=%s\nactivation_policy=%ld\nactive=%s\n",
           visible ? "true" : "false", frontmost ? "true" : "false",
           (long)activation_policy, active ? "true" : "false");
    return 0;
}
