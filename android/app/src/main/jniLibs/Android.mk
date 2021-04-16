LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE            := libsigner
LOCAL_SRC_FILES         := $(TARGET_ARCH_ABI)/libsigner.so
include $(PREBUILT_SHARED_LIBRARY) 

